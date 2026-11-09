use crate::prelude::*;

impl GatewayClient {
    /// What withdrawing `resources` from `account_address` needs: reads the withdraw rule of each
    /// resource (its `withdrawer` role) and, when some are restricted, what the account holds of
    /// the badges they ask for, to find the proofs to create before the withdrawals.
    pub async fn fetch_transfer_withdraw_proofs(
        &self,
        account_address: AccountAddress,
        resources: Vec<ResourceAddress>,
    ) -> Result<TransferWithdrawProofs> {
        let resources = resources.into_iter().unique().collect_vec();
        if resources.is_empty() {
            return Ok(TransferWithdrawProofs::default());
        }

        let withdraw_rules = self
            .batch_fetch_chunking(
                GATEWAY_ENTITY_DETAILS_CHUNK_ADDRESSES,
                resources.clone(),
                StateEntityDetailsRequest::addresses_only,
                |req| self.state_entity_details(req),
                |responses| {
                    Ok(responses
                        .into_iter()
                        .flat_map(|response| response.items)
                        .filter_map(|item| {
                            let resource = *item.address.as_resource()?;
                            let rule = item.details?.withdrawer_rule()?;
                            Some((resource, rule))
                        })
                        .filter(|(_, rule)| *rule != AccessRule::AllowAll)
                        .collect_vec())
                },
            )
            .await?;
        if withdraw_rules.is_empty() {
            return Ok(TransferWithdrawProofs::default());
        }

        let badges = withdraw_rules
            .iter()
            .flat_map(|(_, rule)| rule.proof_resources())
            .unique()
            .collect_vec();
        let holdings =
            self.fetch_proof_holdings(account_address, badges).await?;
        Ok(TransferWithdrawProofs::new(withdraw_rules, &holdings))
    }

    /// What `account_address` holds of the `badges`: the amount of the fungible ones, and the
    /// first page of ids of the non-fungible ones.
    async fn fetch_proof_holdings(
        &self,
        account_address: AccountAddress,
        badges: Vec<ResourceAddress>,
    ) -> Result<ProofHoldings> {
        if badges.is_empty() {
            return Ok(ProofHoldings::default());
        }
        let ledger_state = self.gateway_status().await?.ledger_state;
        let owned = self
            .fetch_all_resources(account_address, ledger_state.clone().into())
            .await?;

        let fungibles = owned
            .fungibles
            .iter()
            .filter_map(|item| item.as_global())
            .filter(|item| badges.contains(&item.resource_address))
            .map(|item| (item.resource_address, item.amount))
            .collect_vec();

        let owned_non_fungible_badges = owned
            .non_fungibles
            .iter()
            .filter_map(|item| item.as_global())
            .filter(|item| {
                badges.contains(&item.resource_address) && item.amount > 0
            })
            .map(|item| item.resource_address)
            .collect_vec();
        let mut non_fungibles = Vec::new();
        for resource in owned_non_fungible_badges {
            let vaults = self
                .state_entity_page_non_fungible_vaults(
                    StateEntityPageNonFungibleVaultsRequest::new(
                        account_address.into(),
                        resource,
                        LedgerStateSelector::from(ledger_state.clone()),
                        None,
                        GATEWAY_PAGE_REQUEST_LIMIT,
                        StateEntityNonFungibleResourceVaultsPageOptIns::include(
                        ),
                    ),
                )
                .await?;
            let ids = vaults
                .items
                .into_iter()
                .flat_map(|vault| vault.items.unwrap_or_default())
                .collect_vec();
            non_fungibles.push((resource, ids));
        }

        Ok(ProofHoldings::new(fungibles, non_fungibles))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use profile_gateway::prelude::Gateway;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = GatewayClient;

    fn badge() -> ResourceAddress {
        ResourceAddress::sample_stokenet_candy()
    }

    fn restricted_token() -> ResourceAddress {
        ResourceAddress::sample_stokenet_gum()
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
                        ComponentEntityRoleAssignmentOwner::new(
                            AccessRule::DenyAll,
                        ),
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

    fn badge_rule() -> AccessRule {
        AccessRule::Protected {
            access_rule: CompositeRequirement::ProofRule {
                proof_rule: BasicRequirement::Require {
                    requirement: Requirement::Resource { resource: badge() },
                },
            },
        }
    }

    fn account_holding_badge(
        account: AccountAddress,
        amount: Decimal192,
    ) -> StateEntityDetailsResponse {
        StateEntityDetailsResponse::new(
            LedgerState::sample(),
            [StateEntityDetailsResponseItem::new(
                account.into(),
                FungibleResourcesCollection::new(
                    1,
                    None,
                    [FungibleResourcesCollectionItem::Global(
                        FungibleResourcesCollectionItemGloballyAggregated::new(
                            badge(),
                            amount,
                        ),
                    )],
                ),
                None,
                EntityMetadataCollection::empty(),
                None,
            )],
        )
    }

    #[actix_rt::test]
    async fn a_restricted_token_gets_a_proof_of_its_badge() {
        let account = AccountAddress::sample_stokenet();
        let mock_driver = MockNetworkingDriver::new_with_responses(vec![
            MockNetworkingDriverResponse::new_success(
                StateEntityDetailsResponse::new(
                    LedgerState::sample(),
                    [resource_details(restricted_token(), badge_rule())],
                ),
            ),
            MockNetworkingDriverResponse::new_success(GatewayStatusResponse {
                ledger_state: LedgerState::sample(),
            }),
            MockNetworkingDriverResponse::new_success(account_holding_badge(
                account,
                Decimal192::from(3u64),
            )),
        ]);
        let sut = SUT::with_networking_driver(
            Arc::new(mock_driver),
            Gateway::stokenet(),
        );

        let result = sut
            .fetch_transfer_withdraw_proofs(account, vec![restricted_token()])
            .await
            .unwrap();

        assert!(result.is_satisfied());
        assert_eq!(
            result.proofs,
            vec![ResourceSpecifier::fungible(badge(), Decimal192::one())]
        );
        assert_eq!(result.restricted_withdrawals[0].badges, vec![badge()]);
    }

    #[actix_rt::test]
    async fn a_missing_badge_is_reported() {
        let account = AccountAddress::sample_stokenet();
        let mock_driver = MockNetworkingDriver::new_with_responses(vec![
            MockNetworkingDriverResponse::new_success(
                StateEntityDetailsResponse::new(
                    LedgerState::sample(),
                    [resource_details(restricted_token(), badge_rule())],
                ),
            ),
            MockNetworkingDriverResponse::new_success(GatewayStatusResponse {
                ledger_state: LedgerState::sample(),
            }),
            MockNetworkingDriverResponse::new_success(account_holding_badge(
                account,
                Decimal192::zero(),
            )),
        ]);
        let sut = SUT::with_networking_driver(
            Arc::new(mock_driver),
            Gateway::stokenet(),
        );

        let result = sut
            .fetch_transfer_withdraw_proofs(account, vec![restricted_token()])
            .await
            .unwrap();

        assert!(!result.is_satisfied());
        assert!(result.proofs.is_empty());
    }

    #[actix_rt::test]
    async fn freely_withdrawable_tokens_need_nothing_more() {
        let account = AccountAddress::sample_stokenet();
        let mock_driver = MockNetworkingDriver::new_with_responses(vec![
            MockNetworkingDriverResponse::new_success(
                StateEntityDetailsResponse::new(
                    LedgerState::sample(),
                    [resource_details(
                        restricted_token(),
                        AccessRule::AllowAll,
                    )],
                ),
            ),
        ]);
        let sut = SUT::with_networking_driver(
            Arc::new(mock_driver),
            Gateway::stokenet(),
        );

        let result = sut
            .fetch_transfer_withdraw_proofs(account, vec![restricted_token()])
            .await
            .unwrap();

        assert_eq!(result, TransferWithdrawProofs::default());
    }
}
