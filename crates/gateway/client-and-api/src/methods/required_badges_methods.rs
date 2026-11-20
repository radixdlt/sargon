use crate::prelude::*;

/// Reads what the withdrawals of a transfer need: the withdraw rules of the transferred resources,
/// and what the account holds of the badges they ask for. Only fetches; matching the rules is done
/// by `TransferRequiredBadges` (in the `manifests` crate).
impl GatewayClient {
    /// The withdraw rule (the ("Main", "withdrawer") role) of each of the `resources`, and the
    /// ledger state they were read at: pin the other requests to it. Fails when the rule of one
    /// of the resources is missing.
    pub async fn fetch_withdraw_rules(
        &self,
        resources: Vec<ResourceAddress>,
    ) -> Result<(LedgerState, Vec<(ResourceAddress, AccessRule)>)> {
        let resources = resources.into_iter().unique().collect_vec();
        let mut ledger_state: Option<LedgerState> = None;
        let mut rules = Vec::new();
        for chunk in resources.chunks(GATEWAY_ENTITY_DETAILS_CHUNK_ADDRESSES) {
            let response = self
                .state_entity_details(StateEntityDetailsRequest::new(
                    chunk.iter().cloned().map(Address::from).collect(),
                    ledger_state.clone().map(LedgerStateSelector::from),
                    None,
                ))
                .await?;
            rules.extend(response.items.into_iter().filter_map(|item| {
                let resource = *item.address.as_resource()?;
                Some((resource, item.details?.withdrawer_rule()?))
            }));
            if ledger_state.is_none() {
                ledger_state = response.ledger_state;
            }
        }
        // A resource without a readable withdrawer rule must not pass for an unrestricted one.
        if let Some(missing) = resources
            .iter()
            .find(|resource| !rules.iter().any(|(r, _)| r == *resource))
        {
            return Err(CommonError::GWMissingResponseItem {
                item: format!("withdrawer rule of {missing}"),
            });
        }
        let ledger_state = match ledger_state {
            Some(ledger_state) => ledger_state,
            None => self.gateway_status().await?.ledger_state,
        };
        Ok((ledger_state, rules))
    }

    /// The amount of the fungible `resource` in the vaults of `account_address`.
    pub async fn fetch_fungible_vaults_amount(
        &self,
        account_address: AccountAddress,
        resource: ResourceAddress,
        ledger_state: LedgerState,
    ) -> Result<Decimal192> {
        let vaults = self
            .load_all_pages(
                None,
                LedgerStateSelector::from(ledger_state),
                |cursor, ledger_state_selector| {
                    self.state_entity_page_fungible_vaults(
                        StateEntityPageFungibleVaultsRequest::new(
                            account_address.into(),
                            resource,
                            ledger_state_selector,
                            cursor,
                            GATEWAY_PAGE_REQUEST_LIMIT,
                        ),
                    )
                },
            )
            .await?;
        Ok(vaults
            .into_iter()
            .fold(Decimal192::zero(), |total, vault| total + vault.amount))
    }

    /// At least `count` ids (when it holds that many) of the non-fungible `resource` in the
    /// vaults of `account_address`: the first page of ids of each vault, and the next pages only
    /// while more are needed.
    pub async fn fetch_non_fungible_vaults_ids(
        &self,
        account_address: AccountAddress,
        resource: ResourceAddress,
        count: u64,
        ledger_state: LedgerState,
    ) -> Result<IndexSet<NonFungibleLocalId>> {
        let at_ledger_state = LedgerStateSelector::from(ledger_state);
        let enough =
            |ids: &IndexSet<NonFungibleLocalId>| ids.len() as u64 >= count;
        let mut ids = IndexSet::new();
        let mut vaults_cursor: Option<String> = None;
        loop {
            let vaults = self
                .state_entity_page_non_fungible_vaults(
                    StateEntityPageNonFungibleVaultsRequest::new(
                        account_address.into(),
                        resource,
                        at_ledger_state.clone(),
                        vaults_cursor,
                        GATEWAY_PAGE_REQUEST_LIMIT,
                        StateEntityNonFungibleResourceVaultsPageOptIns::include(
                        ),
                    ),
                )
                .await?;
            for vault in vaults.items {
                ids.extend(vault.items.unwrap_or_default());
                let mut ids_cursor = vault.next_cursor;
                while !enough(&ids) {
                    let Some(cursor) = ids_cursor else { break };
                    let page = self
                        .state_entity_page_non_fungible_vault_ids(
                            StateEntityPageNonFungibleVaultIdsRequest::new(
                                account_address.into(),
                                vault.vault_address,
                                resource,
                                at_ledger_state.clone(),
                                cursor,
                                GATEWAY_PAGE_REQUEST_LIMIT,
                            ),
                        )
                        .await?;
                    ids.extend(page.items);
                    ids_cursor = page.next_cursor;
                }
                if enough(&ids) {
                    return Ok(ids);
                }
            }
            vaults_cursor = vaults.next_cursor;
            if vaults_cursor.is_none() {
                return Ok(ids);
            }
        }
    }

    /// Which of the `ids` of the non-fungible `resource` are in a vault of `account_address`.
    pub async fn fetch_non_fungibles_held_by(
        &self,
        account_address: AccountAddress,
        resource: ResourceAddress,
        ids: IndexSet<NonFungibleLocalId>,
        ledger_state: LedgerState,
    ) -> Result<IndexSet<NonFungibleLocalId>> {
        let account = Address::from(account_address);
        let mut held = IndexSet::new();
        let ids = ids.into_iter().collect_vec();
        for chunk in ids.chunks(GATEWAY_CHUNK_NON_FUNGIBLES as usize) {
            let response = self
                .state_non_fungible_location(
                    StateNonFungibleLocationRequest::new(
                        resource,
                        chunk.iter().cloned(),
                        LedgerStateSelector::from(ledger_state.clone()),
                    ),
                )
                .await?;
            held.extend(
                response
                    .non_fungible_ids
                    .into_iter()
                    .filter(|location| {
                        !location.is_burned
                            && location.owning_vault_global_ancestor_address
                                == Some(account)
                    })
                    .map(|location| location.non_fungible_id),
            );
        }
        Ok(held)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use profile_gateway::prelude::Gateway;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = GatewayClient;

    fn sut(responses: Vec<MockNetworkingDriverResponse>) -> SUT {
        sut_with_spy(responses, |_, _| {})
    }

    fn sut_with_spy(
        responses: Vec<MockNetworkingDriverResponse>,
        spy: fn(NetworkRequest, u64),
    ) -> SUT {
        SUT::with_networking_driver(
            Arc::new(MockNetworkingDriver::new_with_responses_and_spy(
                responses, spy,
            )),
            Gateway::stokenet(),
        )
    }

    fn account() -> AccountAddress {
        AccountAddress::sample_stokenet()
    }

    fn badge() -> ResourceAddress {
        ResourceAddress::sample_stokenet_candy()
    }

    fn nft_badge() -> ResourceAddress {
        ResourceAddress::sample_stokenet_nft_gc_membership()
    }

    fn ids(ids: &[u64]) -> IndexSet<NonFungibleLocalId> {
        ids.iter()
            .map(|i| NonFungibleLocalId::integer(*i))
            .collect()
    }

    /// The JSON body of a request.
    fn body(request: &NetworkRequest) -> serde_json::Value {
        serde_json::from_slice(request.body.bytes()).unwrap()
    }

    /// The state version a request is pinned to, if any.
    fn pinned_state_version(request: &NetworkRequest) -> Option<u64> {
        body(request)["at_ledger_state"]["state_version"].as_u64()
    }

    fn assert_request(
        request: &NetworkRequest,
        path: &str,
        state_version: Option<u64>,
    ) {
        assert!(
            request.url.as_str().ends_with(path),
            "unexpected request to {}",
            request.url
        );
        assert_eq!(pinned_state_version(request), state_version);
    }

    fn resource_details(
        resource: ResourceAddress,
        withdrawer: AccessRule,
    ) -> StateEntityDetailsResponseItem {
        StateEntityDetailsResponseItem::new(
            resource.into(),
            None,
            None,
            EntityMetadataCollection::empty(),
            StateEntityDetailsResponseItemDetails::FungibleResource(
                StateEntityDetailsResponseFungibleResourceDetails::new(
                    ComponentEntityRoleAssignments::new(
                        ComponentEntityRoleAssignmentOwner::sample_deny_all(),
                        [ComponentEntityRoleAssignmentEntry::new(
                            RoleKey::main_withdrawer(),
                            ComponentEntityRoleAssignmentEntryAssignment::new(
                                RoleAssignmentResolution::Explicit,
                                withdrawer,
                            ),
                        )],
                    ),
                ),
            ),
        )
    }

    #[actix_rt::test]
    async fn withdraw_rules_and_the_ledger_state_they_were_read_at() {
        let restricted = ResourceAddress::sample_stokenet_gum();
        let sut = sut_with_spy(
            vec![MockNetworkingDriverResponse::new_success(
                StateEntityDetailsResponse::new(
                    LedgerState::sample_other(),
                    [
                        resource_details(restricted, AccessRule::DenyAll),
                        resource_details(badge(), AccessRule::AllowAll),
                    ],
                ),
            )],
            |request, _| assert_request(&request, "state/entity/details", None),
        );

        let (ledger_state, rules) = sut
            .fetch_withdraw_rules(vec![restricted, badge(), restricted])
            .await
            .unwrap();

        assert_eq!(ledger_state, LedgerState::sample_other());
        assert_eq!(
            rules,
            vec![
                (restricted, AccessRule::DenyAll),
                (badge(), AccessRule::AllowAll)
            ]
        );
    }

    #[actix_rt::test]
    async fn a_resource_without_a_withdraw_rule_is_an_error() {
        let restricted = ResourceAddress::sample_stokenet_gum();
        let sut = sut(vec![MockNetworkingDriverResponse::new_success(
            StateEntityDetailsResponse::new(
                LedgerState::sample(),
                [resource_details(badge(), AccessRule::AllowAll)],
            ),
        )]);

        let result = sut.fetch_withdraw_rules(vec![restricted, badge()]).await;

        assert_eq!(
            result,
            Err(CommonError::GWMissingResponseItem {
                item: format!("withdrawer rule of {restricted}")
            })
        );
    }

    #[actix_rt::test]
    async fn the_next_chunks_of_withdraw_rules_are_pinned_to_the_first() {
        let resources = (0..GATEWAY_ENTITY_DETAILS_CHUNK_ADDRESSES + 1)
            .map(|_| ResourceAddress::random(NetworkID::Stokenet))
            .collect_vec();
        let sut = sut_with_spy(
            vec![
                MockNetworkingDriverResponse::new_success(
                    StateEntityDetailsResponse::new(
                        LedgerState::sample_other(),
                        resources[..GATEWAY_ENTITY_DETAILS_CHUNK_ADDRESSES]
                            .iter()
                            .map(|r| {
                                resource_details(*r, AccessRule::AllowAll)
                            }),
                    ),
                ),
                MockNetworkingDriverResponse::new_success(
                    StateEntityDetailsResponse::new(
                        LedgerState::sample_other(),
                        [resource_details(
                            *resources.last().unwrap(),
                            AccessRule::DenyAll,
                        )],
                    ),
                ),
            ],
            |request, count| {
                let state_version = (count > 0)
                    .then_some(LedgerState::sample_other().state_version);
                assert_request(&request, "state/entity/details", state_version)
            },
        );

        let (_, rules) =
            sut.fetch_withdraw_rules(resources.clone()).await.unwrap();

        assert_eq!(rules.len(), resources.len());
        assert_eq!(
            rules.last(),
            Some(&(*resources.last().unwrap(), AccessRule::DenyAll))
        );
    }

    #[actix_rt::test]
    async fn fungible_vaults_amount_adds_up_every_vault_of_every_page() {
        let vault = |amount: u64| {
            FungibleResourcesCollectionItemVaultAggregatedVaultItem::new(
                VaultAddress::sample_stokenet_fungible(),
                Decimal192::from(amount),
            )
        };
        let sut = sut_with_spy(
            vec![
                MockNetworkingDriverResponse::new_success(PageResponse::new(
                    LedgerState::sample(),
                    3,
                    "next".to_owned(),
                    vec![vault(1), vault(2)],
                )),
                MockNetworkingDriverResponse::new_success(PageResponse::new(
                    LedgerState::sample(),
                    3,
                    None,
                    vec![vault(4)],
                )),
            ],
            |request, _| {
                assert_request(
                    &request,
                    "state/entity/page/fungible-vaults/",
                    Some(LedgerState::sample().state_version),
                );
                assert_eq!(
                    body(&request)["resource_address"],
                    badge().to_string()
                );
            },
        );

        let amount = sut
            .fetch_fungible_vaults_amount(
                account(),
                badge(),
                LedgerState::sample(),
            )
            .await
            .unwrap();

        assert_eq!(amount, Decimal192::from(7u64));
    }

    fn non_fungible_vault(
        vault_ids: &[u64],
        next_cursor: impl Into<Option<String>>,
    ) -> NonFungibleResourcesCollectionItemVaultAggregatedVaultItem {
        NonFungibleResourcesCollectionItemVaultAggregatedVaultItem {
            total_count: None,
            vault_address: VaultAddress::sample_stokenet_non_fungible(),
            last_updated_at_state_version: 1,
            next_cursor: next_cursor.into(),
            items: Some(ids(vault_ids).into_iter().collect()),
        }
    }

    fn non_fungible_vaults_page(
        vaults: Vec<NonFungibleResourcesCollectionItemVaultAggregatedVaultItem>,
    ) -> MockNetworkingDriverResponse {
        MockNetworkingDriverResponse::new_success(PageResponse::new(
            LedgerState::sample(),
            vaults.len() as u64,
            None,
            vaults,
        ))
    }

    #[actix_rt::test]
    async fn non_fungible_vault_ids_stop_at_the_first_page_when_it_has_enough()
    {
        // A second request would fail: the mock has no response for it.
        let sut =
            sut(vec![non_fungible_vaults_page(vec![non_fungible_vault(
                &[1, 2],
                "more".to_owned(),
            )])]);

        let held = sut
            .fetch_non_fungible_vaults_ids(
                account(),
                nft_badge(),
                2,
                LedgerState::sample(),
            )
            .await
            .unwrap();

        assert_eq!(held, ids(&[1, 2]));
    }

    #[actix_rt::test]
    async fn non_fungible_vault_ids_are_paged_while_more_are_needed() {
        let sut = sut_with_spy(
            vec![
                non_fungible_vaults_page(vec![non_fungible_vault(
                    &[1, 2],
                    "more".to_owned(),
                )]),
                MockNetworkingDriverResponse::new_success(PageResponse::new(
                    LedgerState::sample(),
                    4,
                    None,
                    ids(&[3, 4]).into_iter().collect_vec(),
                )),
            ],
            |request, count| {
                let path = match count {
                    0 => "state/entity/page/non-fungible-vaults/",
                    1 => "state/entity/page/non-fungible-vault/ids",
                    _ => panic!("unexpected request #{count}"),
                };
                assert_request(
                    &request,
                    path,
                    Some(LedgerState::sample().state_version),
                );
                if count == 1 {
                    assert_eq!(body(&request)["cursor"], "more");
                }
            },
        );

        let held = sut
            .fetch_non_fungible_vaults_ids(
                account(),
                nft_badge(),
                3,
                LedgerState::sample(),
            )
            .await
            .unwrap();

        assert_eq!(held, ids(&[1, 2, 3, 4]));
    }

    #[actix_rt::test]
    async fn non_fungible_vault_ids_of_every_vault_count() {
        let sut = sut(vec![non_fungible_vaults_page(vec![
            non_fungible_vault(&[1], None),
            non_fungible_vault(&[5], None),
        ])]);

        let held = sut
            .fetch_non_fungible_vaults_ids(
                account(),
                nft_badge(),
                3,
                LedgerState::sample(),
            )
            .await
            .unwrap();

        // Not as many as asked for: that's all the account holds.
        assert_eq!(held, ids(&[1, 5]));
    }

    #[actix_rt::test]
    async fn non_fungibles_held_by_the_account_are_found_by_their_location() {
        let location = |id: u64, owner: AccountAddress, is_burned: bool| {
            StateNonFungibleLocationResponseItem {
                non_fungible_id: NonFungibleLocalId::integer(id),
                is_burned,
                last_updated_at_state_version: 1,
                owning_vault_address:
                    VaultAddress::sample_stokenet_non_fungible(),
                owning_vault_parent_ancestor_address: Some(owner.into()),
                owning_vault_global_ancestor_address: Some(owner.into()),
            }
        };
        let sut = sut_with_spy(
            vec![MockNetworkingDriverResponse::new_success(
                StateNonFungibleLocationResponse {
                    ledger_state: LedgerState::sample(),
                    resource_address: nft_badge(),
                    non_fungible_ids: vec![
                        location(30, account(), false),
                        location(
                            31,
                            AccountAddress::sample_stokenet_other(),
                            false,
                        ),
                        location(32, account(), true),
                    ],
                },
            )],
            |request, _| {
                assert_request(
                    &request,
                    "state/non-fungible/location",
                    Some(LedgerState::sample().state_version),
                );
                assert_eq!(
                    body(&request)["non_fungible_ids"],
                    serde_json::json!(["#30#", "#31#", "#32#"])
                );
            },
        );

        let held = sut
            .fetch_non_fungibles_held_by(
                account(),
                nft_badge(),
                ids(&[30, 31, 32]),
                LedgerState::sample(),
            )
            .await
            .unwrap();

        assert_eq!(held, ids(&[30]));
    }
}
