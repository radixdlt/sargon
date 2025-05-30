use crate::prelude::*;

use radix_common::prelude::ACCOUNT_OWNER_BADGE as SCRYPTO_ACCOUNT_OWNER_BADGE;
use radix_common::prelude::IDENTITY_OWNER_BADGE as SCRYPTO_IDENTITY_OWNER_BADGE;

impl GatewayClient {
    /// Fetched the XRD balance of account of `address`, returns `None` if
    /// it has no balance.
    pub async fn xrd_balance_of_account(
        &self,
        address: AccountAddress,
    ) -> Result<Option<Decimal192>> {
        let map = self
            .xrd_balances_of_accounts(address.network_id(), vec![address])
            .await?;
        Ok(map.get(&address).cloned().flatten())
    }

    /// Fetches the XRD balance of each account `address`. Returns the
    /// balance for each account (being `None` if it has no balance).
    pub async fn xrd_balances_of_accounts(
        &self,
        network_id: NetworkID,
        addresses: impl IntoIterator<Item = AccountAddress>,
    ) -> Result<IndexMap<AccountAddress, Option<Decimal192>>> {
        let map = self
            .xrd_balances_of_access_controller_or_account(
                network_id,
                addresses
                    .into_iter()
                    .map(AddressOfAccessControllerOrAccount::from),
            )
            .await?;

        let map = map
            .into_iter()
            .map(|(k, v)| {
                (
                    AccountAddress::try_from(k)
                        .expect("should not have received other components"),
                    v,
                )
            })
            .collect();

        Ok(map)
    }

    /// Fetched the XRD balance of the component - either AccountAddress or AccessControllerAddress.
    pub async fn xrd_balances_of_access_controller_or_account(
        &self,
        network_id: NetworkID,
        addresses: impl IntoIterator<Item = AddressOfAccessControllerOrAccount>,
    ) -> Result<IndexMap<AddressOfAccessControllerOrAccount, Option<Decimal192>>>
    {
        let addresses = addresses.into_iter().collect::<IndexSet<_>>();
        let target_address_len = addresses.len();
        self.batch_fetch_chunking(
            GATEWAY_ENTITY_DETAILS_CHUNK_ADDRESSES,
            addresses,
            StateEntityDetailsRequest::addresses_only,
            |req| self.state_entity_details(req),
            |responses| {
                let xrd_address = ResourceAddress::xrd_on_network(network_id);

                let map = responses
                    .into_iter()
                    .flat_map(|response| {
                        response
                            .items
                            .into_iter()
                            .map(|response_item| {
                                let owner = AddressOfAccessControllerOrAccount::try_from(
                                    response_item.address,
                                )
                                .expect("address is valid");

                                let Some(fungible_resources) =
                                    response_item.fungible_resources
                                else {
                                    return Ok((owner, None));
                                };

                                if let Some(xrd_resource_collection_item) =
                                    fungible_resources.items.into_iter().find(
                                        |x| x.resource_address() == xrd_address,
                                    )
                                {
                                    let xrd_resource =
                                        xrd_resource_collection_item
                                            .as_global()
                                            .expect("Global is default");
                                    Ok((owner, Some(xrd_resource.amount)))
                                } else {
                                    Ok((owner, None))
                                }
                            })
                            .collect::<Vec<Result<_>>>()
                    })
                    .collect::<Result<IndexMap<_, _>>>()?;

                debug_assert_eq!(
                    map.len(),
                    target_address_len,
                    "Gateway did not respond with all requested addresses"
                );

                Ok(map)
            },
        )
        .await
    }

    /// Fetched the XRD balance of account of `address`, returns `0` if
    /// it has no balance.
    pub async fn xrd_balance_of_account_or_zero(
        &self,
        address: AccountAddress,
    ) -> Result<Decimal192> {
        self.xrd_balance_of_account(address)
            .await
            .map(|x| x.unwrap_or(Decimal192::zero()))
    }

    /// Fetches the badge owner for each entity.
    ///
    /// Each entity is controlled by an owner badge NFT which can be constructed by its address.
    /// By fetching this badge's location on ledger we can retrieve the owner of that badge.
    /// For now the sargon cares about two kinds of owners:
    /// - An account address as an owner:
    ///   Means that the entity's badge owner is the entity itself. This entity is deleted.
    /// - An access controller as an owner:
    ///   Means that this entity is securified and is controlled ny that access controller.
    pub async fn fetch_entities_badge_owners(
        &self,
        network_id: NetworkID,
        entity_addresses: impl IntoIterator<Item = AddressOfAccountOrPersona>,
    ) -> Result<HashMap<AddressOfAccountOrPersona, Option<Address>>> {
        // Construct the owner badge resource address
        let account_owner_badge_resource_address =
            ResourceAddress::new_from_node_id(
                SCRYPTO_ACCOUNT_OWNER_BADGE,
                network_id,
            )?;

        // Construct the owner badge resource address
        let identity_owner_badge_resource_address =
            ResourceAddress::new_from_node_id(
                SCRYPTO_IDENTITY_OWNER_BADGE,
                network_id,
            )?;

        let fetch_ancestors = async |owner_badge: ResourceAddress,
                                     addresses: Vec<
            AddressOfAccountOrPersona,
        >|
               -> Result<
            HashMap<AddressOfAccountOrPersona, Option<Address>>,
        > {
            // Break entities into chunks
            let address_chunks = addresses
                .into_iter()
                .chunks(GATEWAY_CHUNK_NON_FUNGIBLES as usize)
                .into_iter()
                .map(|c| c.collect_vec())
                .collect_vec();

            let mut result =
                HashMap::<AddressOfAccountOrPersona, Option<Address>>::new();
            for chunk in address_chunks.into_iter() {
                // Construct supposed badges for each account
                let badges = chunk
                    .into_iter()
                    .map(|a| (NonFungibleLocalId::from(a), a))
                    .collect::<HashMap<NonFungibleLocalId, AddressOfAccountOrPersona>>();

                // Query the location of the badges
                let non_fungible_ids_location = self
                    .state_non_fungible_location(
                        StateNonFungibleLocationRequest::new(
                            owner_badge,
                            badges.keys().cloned().collect_vec(),
                            None,
                        ),
                    )
                    .await?
                    .non_fungible_ids;

                // Extract for each badge the parent address entity (if exists)
                non_fungible_ids_location.iter().for_each(|location| {
                    let id = location.clone().non_fungible_id;
                    let parent = location.owning_vault_global_ancestor_address;

                    if let Some(entity_address) = badges.get(&id).cloned() {
                        result.insert(entity_address, parent);
                    }
                });
            }

            Ok(result)
        };

        let all_addresses = entity_addresses.into_iter().collect_vec();
        let mut ancestors =
            HashMap::<AddressOfAccountOrPersona, Option<Address>>::new();

        ancestors.extend(
            fetch_ancestors(
                account_owner_badge_resource_address,
                all_addresses
                    .clone()
                    .into_iter()
                    .filter(|a| a.is_account())
                    .collect_vec(),
            )
            .await?,
        );

        ancestors.extend(
            fetch_ancestors(
                identity_owner_badge_resource_address,
                all_addresses
                    .into_iter()
                    .filter(|a| a.is_identity())
                    .collect_vec(),
            )
            .await?,
        );

        Ok(ancestors)
    }

    /// Looks up on ledger whether this `account_address` is deleted, by looking up the NFTs
    /// it owns and checking if its owner badge is one of them.
    pub async fn check_accounts_are_deleted(
        &self,
        network_id: NetworkID,
        account_addresses: impl IntoIterator<Item = AccountAddress>,
    ) -> Result<IndexMap<AccountAddress, bool>> {
        let requested_addresses = account_addresses.into_iter().collect_vec();
        let global_ancestor_address_per_account = self
            .fetch_entities_badge_owners(
                network_id,
                requested_addresses
                    .clone()
                    .iter()
                    .map(|a| AddressOfAccountOrPersona::from(*a)),
            )
            .await?;

        let mut result = IndexMap::<AccountAddress, bool>::new();
        requested_addresses.iter().for_each(|address| {
            if let Some(ancestor_address) = global_ancestor_address_per_account
                .get(&AddressOfAccountOrPersona::from(*address))
                .cloned()
                .unwrap_or(None)
            {
                let is_ancestor_account_address = ancestor_address
                    .into_account()
                    .map(|a| a == *address)
                    .unwrap_or(false);

                result.insert(*address, is_ancestor_account_address);
            } else {
                result.insert(*address, false);
            }
        });

        Ok(result)
    }
}

impl GatewayClient {
    /// Fetches the metadata for the given entity.
    pub async fn fetch_entity_metadata(
        &self,
        address: Address,
        explicit_metadata: Vec<MetadataKey>,
    ) -> Result<EntityMetadataCollection> {
        let response = self
            .state_entity_details(StateEntityDetailsRequest::address_metadata(
                address,
                explicit_metadata,
            ))
            .await?;

        let Some(response_item) =
            response.items.into_iter().find(|x| x.address == address)
        else {
            return Err(CommonError::EntityNotFound);
        };

        Ok(response_item.metadata)
    }

    /// Fetches the metadata for the given dapp.
    pub async fn fetch_dapp_metadata(
        &self,
        address: DappDefinitionAddress,
    ) -> Result<EntityMetadataCollection> {
        self.fetch_entity_metadata(address.into(), dapp_metadata_keys())
            .await
    }
}

impl GatewayClient {
    /// Fetches all the account's resource preferences.
    pub async fn fetch_all_account_resource_preferences(
        &self,
        account_address: AccountAddress,
        ledger_state_selector: LedgerStateSelector,
    ) -> Result<Vec<AccountResourcePreference>> {
        self.load_all_pages(
            None,
            ledger_state_selector,
            |cursor, ledger_state_selector| {
                let request = AccountPageResourcePreferencesRequest::new(
                    account_address,
                    ledger_state_selector,
                    cursor,
                    GATEWAY_PAGE_REQUEST_LIMIT,
                );
                self.account_page_resource_preferences(request)
            },
        )
        .await
    }

    /// Fetches all the account's authorized depositors.
    pub async fn fetch_all_account_authorized_depositors(
        &self,
        account_address: AccountAddress,
        ledger_state_selector: LedgerStateSelector,
    ) -> Result<Vec<AccountAuthorizedDepositor>> {
        self.load_all_pages(
            None,
            ledger_state_selector,
            |cursor, ledger_state_selector| {
                let request = AccountPageAuthorizedDepositorsRequest::new(
                    account_address,
                    ledger_state_selector,
                    cursor,
                    GATEWAY_PAGE_REQUEST_LIMIT,
                );
                self.account_page_authorized_depositors(request)
            },
        )
        .await
    }
}

impl GatewayClient {
    pub async fn fetch_all_resources(
        &self,
        account_address: AccountAddress,
        ledger_state_selector: LedgerStateSelector,
    ) -> Result<FetchResourcesOutput> {
        // Get entity details
        let address = Address::from(account_address);
        let response = self
            .state_entity_details(
                StateEntityDetailsRequest::address_ledger_state(
                    address,
                    ledger_state_selector,
                ),
            )
            .await?;

        // Find the corresponding entity among the response.
        let Some(details) = response
            .clone()
            .items
            .into_iter()
            .find(|x| x.address == address)
        else {
            return Err(CommonError::EntityNotFound);
        };

        // Get the LedgerStateSelector from the response
        let ledger_state_selector: Option<LedgerStateSelector> =
            response.ledger_state.map(Into::into);

        // Fetch all fungible items
        let fungibles =
            if let Some(collection) = details.clone().fungible_resources {
                self.fetch_all_fungible_items(
                    collection,
                    address,
                    ledger_state_selector.clone(),
                )
                .await?
            } else {
                Vec::new()
            };

        // Fetch all non_fungible items
        let non_fungibles =
            if let Some(collection) = details.clone().non_fungible_resources {
                self.fetch_all_non_fungible_items(
                    collection,
                    address,
                    ledger_state_selector.clone(),
                )
                .await?
            } else {
                Vec::new()
            };

        let output = FetchResourcesOutput::new(fungibles, non_fungibles);
        Ok(output)
    }

    /// Given a `FungibleResourcesCollection`, fetches all the remaining pages to get all the
    /// resources for the given `Address` and `LedgerStateSelector`. If there are no more pages to
    /// load, it will return the list of items provided in the first page (this is, the collection).
    ///
    /// Returns: the list with all the `FungibleResourcesCollectionItem`.
    async fn fetch_all_fungible_items(
        &self,
        collection: FungibleResourcesCollection,
        address: Address,
        ledger_state_selector: Option<LedgerStateSelector>,
    ) -> Result<Vec<FungibleResourcesCollectionItem>> {
        let mut items = collection.items;
        if let Some(next_cursor) = collection.next_cursor {
            let remaining = self
                .load_all_pages(
                    next_cursor,
                    ledger_state_selector,
                    |cursor, ledger_state_selector| {
                        let request = StateEntityPageFungiblesRequest::new(
                            address,
                            ledger_state_selector,
                            cursor,
                            GATEWAY_PAGE_REQUEST_LIMIT,
                        );
                        self.state_entity_page_fungibles(request)
                    },
                )
                .await?;
            items.extend(remaining);
        }

        Ok(items)
    }

    /// Given a `NonFungibleResourcesCollection`, fetches all the remaining pages to get all the
    /// resources for the given `Address` and `LedgerStateSelector`. If there are no more pages to
    /// load, it will return the list of items provided in the first page (this is, the collection).
    ///
    /// Returns: the list with all the `NonFungibleResourcesCollectionItem`.
    async fn fetch_all_non_fungible_items(
        &self,
        collection: NonFungibleResourcesCollection,
        address: Address,
        ledger_state_selector: Option<LedgerStateSelector>,
    ) -> Result<Vec<NonFungibleResourcesCollectionItem>> {
        let mut items = collection.items;
        if let Some(next_cursor) = collection.next_cursor {
            let remaining = self
                .load_all_pages(
                    next_cursor,
                    ledger_state_selector,
                    |cursor, ledger_state_selector| {
                        let request = StateEntityPageNonFungiblesRequest::new(
                            address,
                            ledger_state_selector,
                            cursor,
                            GATEWAY_PAGE_REQUEST_LIMIT,
                        );
                        self.state_entity_page_non_fungibles(request)
                    },
                )
                .await?;
            items.extend(remaining);
        }

        Ok(items)
    }
}

impl GatewayClient {
    pub async fn filter_transferable_resources(
        &self,
        output: FetchResourcesOutput,
    ) -> Result<FetchTransferableResourcesOutput> {
        let non_transferable_resources = self
            .batch_fetch_chunking(
                GATEWAY_ENTITY_DETAILS_CHUNK_ADDRESSES,
                output.resource_addresses(),
                StateEntityDetailsRequest::addresses_only,
                |req| self.state_entity_details(req),
                |responses| {
                    // Filter those that cannot be transferred
                    let non_transferable_resources = responses
                        .into_iter()
                        .flat_map(|response| {
                            response
                                .items
                                .into_iter()
                                .filter(|item| !item.can_be_transferred())
                                .filter_map(|item| {
                                    item.address.as_resource().cloned()
                                })
                                .collect_vec()
                        })
                        .collect_vec();
                    Ok(non_transferable_resources)
                },
            )
            .await?;

        // Filter out the fungible and non-fungible items that cannot be transferred
        let fungible = output
            .fungibles
            .into_iter()
            .filter_map(|item| item.as_global().cloned())
            .filter(|item| {
                !(non_transferable_resources.contains(&item.resource_address)
                    || item.amount.is_zero())
            })
            .collect::<Vec<_>>();

        let non_fungible = output
            .non_fungibles
            .into_iter()
            .filter_map(|item| item.as_global().cloned())
            .filter(|item| {
                !(non_transferable_resources.contains(&item.resource_address)
                    || item.amount == 0)
            })
            .collect::<Vec<_>>();

        // Build result
        let result = FetchTransferableResourcesOutput::new(
            fungible,
            non_fungible,
            non_transferable_resources,
        );
        Ok(result)
    }
}

impl GatewayClient {
    pub async fn fetch_non_fungible_data(
        &self,
        collection_address: NonFungibleResourceAddress,
        id: NonFungibleLocalId,
    ) -> Result<NonFungibleTokenData> {
        let request = StateNonFungibleDataRequest::new(
            collection_address,
            [id.clone()],
            None,
        );
        let response = self.state_non_fungible_data(request).await?;
        let item = response.non_fungible_ids.first().ok_or(CommonError::GWMissingResponseItem { item: "StateNonFungibleDataResponseItem".to_owned() })?;
        Ok(NonFungibleTokenData::new(
            NonFungibleGlobalId::new(collection_address, id),
            item.data.clone(),
        ))
    }

    pub async fn fetch_non_fungible_location(
        &self,
        collection_address: NonFungibleResourceAddress,
        id: NonFungibleLocalId,
    ) -> Result<Option<Address>> {
        let request = StateNonFungibleLocationRequest::new(
            collection_address.0,
            vec![id.clone()],
            None,
        );
        let response = self.state_non_fungible_location(request).await?;
        let item = response.non_fungible_ids.first().ok_or(CommonError::GWMissingResponseItem { item: "StateNonFungibleLocationResponseItem".to_owned() })?;
        Ok(item.owning_vault_global_ancestor_address.clone())
    }
}

#[cfg(test)]
mod fetch_all_resources_tests {
    use crate::prelude::*;
    use profile_gateway::prelude::Gateway;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = GatewayClient;

    #[actix_rt::test]
    async fn no_more_pages_to_load() {
        // Test the case where the `state/entity/details` returns all the resources

        // Mock the entity details response
        let account = AccountAddress::sample();
        let fungible = FungibleResourcesCollectionItem::sample();
        let non_fungible = NonFungibleResourcesCollectionItem::sample();
        let entity_details_response = mock_entity_details_response(
            account,
            None,
            None,
            vec![fungible.clone()],
            None,
            vec![non_fungible.clone()],
        );

        // Mock the driver and verify only 1 request is made to the GW
        let mock_driver = MockNetworkingDriver::new_with_responses_and_spy(
            vec![entity_details_response],
            spy_no_more_pages_to_load_requests(),
        );
        let sut = SUT::with_gateway(Arc::new(mock_driver), Gateway::stokenet());

        // Execute the request and check the result
        let result = sut
            .fetch_all_resources(account, LedgerStateSelector::sample())
            .await
            .unwrap();

        assert_eq!(result.fungibles, vec![fungible]);
        assert_eq!(result.non_fungibles, vec![non_fungible]);
    }

    fn spy_no_more_pages_to_load_requests() -> fn(NetworkRequest, u64) {
        |request, count| match count {
            0 => {
                let expected_request =
                    StateEntityDetailsRequest::address_ledger_state(
                        AccountAddress::sample().into(),
                        LedgerStateSelector::sample(),
                    );

                assert_network_request(request, &expected_request);
            }
            _ => {
                panic!("Unexpected request count: {}", count);
            }
        }
    }

    #[actix_rt::test]
    async fn more_pages_to_load() {
        // Test the case where the `state/entity/details` returns the first page of resources, but
        // we need to load one more page for fungibles and one more page for non-fungibles.

        // Mock the entity details response
        let account = AccountAddress::sample();
        let ledger_state = LedgerState::sample();
        let fungible_one = FungibleResourcesCollectionItem::sample();
        let fungible_cursor = "next_fungible_cursor".to_string();
        let non_fungible_one = NonFungibleResourcesCollectionItem::sample();
        let non_fungible_cursor = "next_non_fungible_cursor".to_string();
        let entity_details_response = mock_entity_details_response(
            account,
            ledger_state,
            fungible_cursor,
            vec![fungible_one.clone()],
            non_fungible_cursor,
            vec![non_fungible_one.clone()],
        );

        // Mock the fungibles page response
        let fungible_two = FungibleResourcesCollectionItem::sample_other();
        let fungibles_page_response =
            mock_fungibles_page_response(vec![fungible_two.clone()]);

        // Mock the non-fungibles page response
        let non_fungible_two =
            NonFungibleResourcesCollectionItem::sample_other();
        let non_fungibles_page_response =
            mock_non_fungibles_page_response(vec![non_fungible_two.clone()]);

        // Mock the driver and verify the 3 expected requests are made to the GW
        let mock_driver = MockNetworkingDriver::new_with_responses_and_spy(
            vec![
                entity_details_response,
                fungibles_page_response,
                non_fungibles_page_response,
            ],
            spy_more_pages_to_load_requests(),
        );
        let sut = SUT::with_gateway(Arc::new(mock_driver), Gateway::stokenet());

        // Execute the request and check the result
        let result = sut
            .fetch_all_resources(account, LedgerStateSelector::sample())
            .await
            .unwrap();

        assert_eq!(result.fungibles, vec![fungible_one, fungible_two]);
        assert_eq!(
            result.non_fungibles,
            vec![non_fungible_one, non_fungible_two]
        );
    }

    fn spy_more_pages_to_load_requests() -> fn(NetworkRequest, u64) {
        |request, count| match count {
            0 => {
                let expected_request =
                    StateEntityDetailsRequest::address_ledger_state(
                        AccountAddress::sample().into(),
                        LedgerStateSelector::sample(),
                    );

                assert_network_request(request, &expected_request);
            }
            1 => {
                let expected_request = StateEntityPageFungiblesRequest::new(
                    AccountAddress::sample().into(),
                    LedgerStateSelector::from(LedgerState::sample()),
                    "next_fungible_cursor".to_string(),
                    GATEWAY_PAGE_REQUEST_LIMIT,
                );

                assert_network_request(request, &expected_request);
            }
            2 => {
                let expected_request = StateEntityPageNonFungiblesRequest::new(
                    AccountAddress::sample().into(),
                    LedgerStateSelector::from(LedgerState::sample()),
                    "next_non_fungible_cursor".to_string(),
                    GATEWAY_PAGE_REQUEST_LIMIT,
                );

                assert_network_request(request, &expected_request);
            }
            _ => {
                panic!("Unexpected request count: {}", count);
            }
        }
    }

    #[actix_rt::test]
    async fn entity_with_no_resources() {
        // Test the case where the `state/entity/details` returns an entity with no resources.

        // Mock the entity details response
        let account = AccountAddress::sample();
        let item = StateEntityDetailsResponseItem::new(
            account.into(),
            None,
            None,
            EntityMetadataCollection::empty(),
            None,
        );
        let response = MockNetworkingDriverResponse::new_success(
            StateEntityDetailsResponse::new(LedgerState::sample(), vec![item]),
        );

        // Mock the driver
        let mock_driver =
            MockNetworkingDriver::new_with_responses(vec![response]);
        let sut = SUT::with_gateway(Arc::new(mock_driver), Gateway::stokenet());

        // Execute the request and check the result has two empty collections
        let result = sut
            .fetch_all_resources(account, LedgerStateSelector::sample())
            .await
            .unwrap();

        assert!(result.fungibles.is_empty());
        assert!(result.non_fungibles.is_empty());
    }

    #[actix_rt::test]
    async fn entity_not_found() {
        // Test the case where the `state/entity/details` doesn't return the entity we are looking for.

        // Mock the entity details response
        let account = AccountAddress::sample();
        let other_account = AccountAddress::sample_other();
        let entity_details_response = mock_entity_details_response(
            other_account,
            None,
            None,
            vec![],
            None,
            vec![],
        );

        // Mock the driver
        let mock_driver = MockNetworkingDriver::new_with_responses(vec![
            entity_details_response,
        ]);
        let sut = SUT::with_gateway(Arc::new(mock_driver), Gateway::stokenet());

        // Execute the request and check the result is a failure
        let result = sut
            .fetch_all_resources(account, LedgerStateSelector::sample())
            .await
            .expect_err("Expected an error");

        assert_eq!(result, CommonError::EntityNotFound);
    }

    /// Creates a `MockNetworkingDriverResponse` for a `StateEntityDetailsResponse`.
    fn mock_entity_details_response(
        account_address: AccountAddress,
        ledger_state: impl Into<Option<LedgerState>>,
        fungible_next_cursor: impl Into<Option<String>>,
        fungible_items: Vec<FungibleResourcesCollectionItem>,
        non_fungible_next_cursor: impl Into<Option<String>>,
        non_fungible_items: Vec<NonFungibleResourcesCollectionItem>,
    ) -> MockNetworkingDriverResponse {
        let fungible_collection = FungibleResourcesCollection::new(
            None,
            fungible_next_cursor,
            fungible_items,
        );
        let non_fungible_collection = NonFungibleResourcesCollection::new(
            None,
            non_fungible_next_cursor,
            non_fungible_items,
        );
        let item = StateEntityDetailsResponseItem::new(
            account_address.into(),
            fungible_collection,
            non_fungible_collection,
            EntityMetadataCollection::empty(),
            None,
        );
        MockNetworkingDriverResponse::new_success(
            StateEntityDetailsResponse::new(ledger_state, vec![item]),
        )
    }

    /// Creates a `MockNetworkingDriverResponse` for a fungibles `PageResponse`.
    fn mock_fungibles_page_response(
        items: Vec<FungibleResourcesCollectionItem>,
    ) -> MockNetworkingDriverResponse {
        MockNetworkingDriverResponse::new_success(PageResponse::new(
            LedgerState::sample(),
            2,
            None,
            items,
        ))
    }

    /// Creates a `MockNetworkingDriverResponse` for a non-fungibles `PageResponse`.
    fn mock_non_fungibles_page_response(
        items: Vec<NonFungibleResourcesCollectionItem>,
    ) -> MockNetworkingDriverResponse {
        MockNetworkingDriverResponse::new_success(PageResponse::new(
            LedgerState::sample(),
            2,
            None,
            items,
        ))
    }
}

#[cfg(test)]
mod filter_transferable_tests {
    use crate::prelude::*;
    use profile_gateway::prelude::Gateway;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = GatewayClient;

    #[actix_rt::test]
    async fn filter_resources() {
        // Test the case where all resources are transferable

        // Mock the resources
        let fungible_transferable = FungibleResourcesCollectionItem::sample();
        let fungible_not_transferable =
            FungibleResourcesCollectionItem::sample_other();
        let fungible_zero_amount = FungibleResourcesCollectionItem::Global(
            FungibleResourcesCollectionItemGloballyAggregated::new(
                ResourceAddress::sample_stokenet_gum(),
                Decimal192::zero(),
            ),
        );
        let non_fungible_transferable =
            NonFungibleResourcesCollectionItem::sample();
        let non_fungible_not_transferable =
            NonFungibleResourcesCollectionItem::sample_other();
        let non_fungible_zero_amount =
            NonFungibleResourcesCollectionItem::Global(
                NonFungibleResourcesCollectionItemGloballyAggregated::new(
                    ResourceAddress::sample_stokenet_nft_gc_membership(),
                    0,
                ),
            );
        let output = FetchResourcesOutput::new(
            [
                fungible_transferable.clone(),
                fungible_not_transferable.clone(),
                fungible_zero_amount.clone(),
            ],
            [
                non_fungible_transferable.clone(),
                non_fungible_not_transferable.clone(),
                non_fungible_zero_amount.clone(),
            ],
        );

        // Mock the entity details response
        let entity_details_response = mock_entity_details_response([
            (
                fungible_transferable.clone().resource_address(),
                fungible_details(true),
            ),
            (
                fungible_not_transferable.clone().resource_address(),
                fungible_details(false),
            ),
            (
                non_fungible_transferable.clone().resource_address(),
                non_fungible_details(true),
            ),
            (
                non_fungible_not_transferable.clone().resource_address(),
                non_fungible_details(false),
            ),
        ]);

        // Mock the driver and verify only 1 request is made to the GW
        let mock_driver = MockNetworkingDriver::new_with_responses(vec![
            entity_details_response,
        ]);
        let sut = SUT::with_gateway(Arc::new(mock_driver), Gateway::stokenet());

        // Execute the request and check the result
        let result = sut.filter_transferable_resources(output).await.unwrap();

        assert_eq!(
            result.fungibles,
            vec![fungible_transferable.as_global().unwrap().clone()]
        );
        assert_eq!(
            result.non_fungibles,
            vec![non_fungible_transferable.as_global().unwrap().clone()]
        );
        assert_eq!(
            result.non_transferable_resources,
            vec![
                fungible_not_transferable.resource_address(),
                non_fungible_not_transferable.resource_address()
            ]
        );
    }

    /// Creates a `MockNetworkingDriverResponse` for a `StateEntityDetailsResponse`.
    fn mock_entity_details_response(
        address_details: impl IntoIterator<
            Item = (ResourceAddress, StateEntityDetailsResponseItemDetails),
        >,
    ) -> MockNetworkingDriverResponse {
        let mut items: Vec<StateEntityDetailsResponseItem> = vec![];
        for item in address_details {
            items.push(StateEntityDetailsResponseItem::new(
                item.0.into(),
                None,
                None,
                EntityMetadataCollection::empty(),
                item.1,
            ));
        }
        MockNetworkingDriverResponse::new_success(
            StateEntityDetailsResponse::new(LedgerState::sample(), items),
        )
    }

    fn fungible_details(
        is_transferable: bool,
    ) -> StateEntityDetailsResponseItemDetails {
        if is_transferable {
            StateEntityDetailsResponseItemDetails::FungibleResource(
                StateEntityDetailsResponseFungibleResourceDetails::new(
                    ComponentEntityRoleAssignments::sample(),
                ),
            )
        } else {
            StateEntityDetailsResponseItemDetails::FungibleResource(
                StateEntityDetailsResponseFungibleResourceDetails::new(
                    ComponentEntityRoleAssignments::sample_other(),
                ),
            )
        }
    }

    fn non_fungible_details(
        is_transferable: bool,
    ) -> StateEntityDetailsResponseItemDetails {
        if is_transferable {
            StateEntityDetailsResponseItemDetails::NonFungibleResource(
                StateEntityDetailsResponseNonFungibleResourceDetails::new(
                    ComponentEntityRoleAssignments::sample(),
                ),
            )
        } else {
            StateEntityDetailsResponseItemDetails::NonFungibleResource(
                StateEntityDetailsResponseNonFungibleResourceDetails::new(
                    ComponentEntityRoleAssignments::sample_other(),
                ),
            )
        }
    }
}