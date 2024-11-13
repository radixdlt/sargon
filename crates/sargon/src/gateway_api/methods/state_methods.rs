use crate::prelude::*;

impl GatewayClient {
    /// Fetched the XRD balance of account of `address`, returns `None` if
    /// it has no balance.
    pub async fn xrd_balance_of_account(
        &self,
        address: AccountAddress,
    ) -> Result<Option<Decimal192>> {
        let response: StateEntityDetailsResponse = self
            .state_entity_details(StateEntityDetailsRequest::new(
                vec![address.into()],
                None,
                None,
            ))
            .await?;

        let Some(response_item) = response
            .items
            .into_iter()
            .find(|x| x.address == address.into())
        else {
            return Ok(None);
        };

        let fungible_resources = response_item
            .fungible_resources
            .expect("Never None for Account");

        let xrd_address = ResourceAddress::xrd_on_network(address.network_id());

        let Some(xrd_resource_collection_item) = fungible_resources
            .items
            .into_iter()
            .find(|x| x.resource_address() == xrd_address)
        else {
            return Ok(None);
        };

        let xrd_resource = xrd_resource_collection_item
            .as_global()
            .expect("Global is default");

        Ok(Some(xrd_resource.amount))
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

        // Fetch all non-fungible items
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
