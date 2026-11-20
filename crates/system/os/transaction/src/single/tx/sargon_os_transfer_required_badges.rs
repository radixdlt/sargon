use crate::prelude::*;
use futures::future::{join_all, try_join_all};
use futures::FutureExt;

#[async_trait::async_trait]
pub trait OsTransferRequiredBadges {
    async fn transfer_required_badges(
        &self,
        from_account: AccountAddress,
        resources: Vec<ResourceAddress>,
    ) -> Result<TransferRequiredBadges>;
}

// ==================
// Transfer required badges (Public)
// ==================
#[async_trait::async_trait]
impl OsTransferRequiredBadges for SargonOS {
    /// What withdrawing `resources` from `from_account` needs: the badges to prove (give them to
    /// the transfer manifest), and the restricted resources with what their rules ask for (to
    /// show to the person). Reads the withdraw rules of the resources, then only the badges they
    /// ask for, all at the ledger state of the rules.
    async fn transfer_required_badges(
        &self,
        from_account: AccountAddress,
        resources: Vec<ResourceAddress>,
    ) -> Result<TransferRequiredBadges> {
        if resources.is_empty() {
            return Ok(TransferRequiredBadges::default());
        }
        let gateway_client = self.gateway_client();

        // Fetch the withdraw rules, and the ledger state to pin the other requests to
        let (ledger_state, rules) =
            gateway_client.fetch_withdraw_rules(resources).await?;
        let withdraw_rules = WithdrawRules::new(rules)?;
        if withdraw_rules.is_empty() {
            return Ok(TransferRequiredBadges::default());
        }

        // Fetch what the account holds of the badges the rules ask for
        let holdings = fetch_badge_holdings(
            &gateway_client,
            from_account,
            withdraw_rules.badges_to_look_up(),
            ledger_state,
        )
        .await?;

        // Match the rules
        Ok(TransferRequiredBadges::new(
            &withdraw_rules,
            signer_public_keys(self, from_account),
            &holdings,
        ))
    }
}

/// The keys that surely sign the transactions of `account`: the transaction signing key of an
/// unsecured account. None for a securified account (which of its factors sign varies: a subset
/// of the primary role, or the recovery and confirmation roles), nor for an account that isn't
/// in the profile, so that a rule asking for one of their signatures is not claimed satisfied.
fn signer_public_keys(
    os: &SargonOS,
    account: AccountAddress,
) -> Vec<PublicKey> {
    let Ok(account) = os.account_by_address(account) else {
        return Vec::new();
    };
    match &account.security_state {
        EntitySecurityState::Unsecured { value } => {
            vec![value.transaction_signing.public_key()]
        }
        EntitySecurityState::Securified { .. } => Vec::new(),
    }
}

/// What `account_address` holds of the `badges`, at `ledger_state`. The requests run
/// concurrently.
async fn fetch_badge_holdings(
    gateway_client: &GatewayClient,
    account_address: AccountAddress,
    badges: BadgesToLookUp,
    ledger_state: LedgerState,
) -> Result<BadgeHoldings> {
    let fungibles =
        try_join_all(badges.fungibles.into_iter().map(|resource| {
            let ledger_state = ledger_state.clone();
            async move {
                gateway_client
                    .fetch_fungible_vaults_amount(
                        account_address,
                        resource,
                        ledger_state,
                    )
                    .await
                    .map(|amount| (resource, amount))
            }
        }));
    let non_fungibles = join_all(
        badges
            .non_fungibles
            .into_iter()
            .map(|(resource, count)| {
                let ledger_state = ledger_state.clone();
                async move {
                    gateway_client
                        .fetch_non_fungible_vaults_ids(
                            account_address,
                            resource,
                            count,
                            ledger_state,
                        )
                        .await
                        .map(|ids| (resource, ids))
                }
                .boxed()
            })
            .chain(badges.non_fungible_ids.into_iter().map(
                |(resource, ids)| {
                    let ledger_state = ledger_state.clone();
                    async move {
                        gateway_client
                            .fetch_non_fungibles_held_by(
                                account_address,
                                resource,
                                ids,
                                ledger_state,
                            )
                            .await
                            .map(|held| (resource, held))
                    }
                    .boxed()
                },
            )),
    );
    let (fungibles, non_fungibles) =
        futures::future::join(fungibles, non_fungibles).await;

    let non_fungibles =
        non_fungibles.into_iter().collect::<Result<Vec<_>>>()?;
    Ok(BadgeHoldings::new(fungibles?, non_fungibles))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::single::support::prepare_os;

    fn account() -> Account {
        Account::sample_mainnet()
    }

    /// The restricted token being transferred.
    fn token() -> ResourceAddress {
        ResourceAddress::sample_mainnet_xrd()
    }

    fn badge() -> ResourceAddress {
        ResourceAddress::sample_mainnet_candy()
    }

    fn nft_badge() -> ResourceAddress {
        ResourceAddress::sample_mainnet_nft_other()
    }

    fn protected(requirement: Requirement) -> AccessRule {
        AccessRule::Protected {
            access_rule: CompositeRequirement::ProofRule {
                proof_rule: BasicRequirement::Require { requirement },
            },
        }
    }

    fn non_fungible_requirement(id: NonFungibleGlobalId) -> Requirement {
        Requirement::NonFungible {
            non_fungible: NonFungible {
                resource_address: id.resource_address,
                local_id: NonFungibleLocalIdId {
                    id_type: String::new(),
                    sbor_hex: String::new(),
                    simple_rep: id.non_fungible_local_id.to_string(),
                },
            },
        }
    }

    fn withdraw_rules_response(
        rules: impl IntoIterator<Item = (ResourceAddress, AccessRule)>,
    ) -> MockNetworkingDriverResponse {
        let items = rules
            .into_iter()
            .map(|(resource, rule)| {
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
                                        rule,
                                    ),
                                )],
                            ),
                        ),
                    ),
                )
            })
            .collect_vec();
        MockNetworkingDriverResponse::new_success(
            StateEntityDetailsResponse::new(LedgerState::sample(), items),
        )
    }

    fn fungible_vaults_response(amount: u64) -> MockNetworkingDriverResponse {
        MockNetworkingDriverResponse::new_success(PageResponse::new(
            LedgerState::sample(),
            1,
            None,
            vec![
                FungibleResourcesCollectionItemVaultAggregatedVaultItem::new(
                    VaultAddress::sample_mainnet_fungible(),
                    Decimal192::from(amount),
                ),
            ],
        ))
    }

    async fn required_badges(
        responses: Vec<MockNetworkingDriverResponse>,
        resources: Vec<ResourceAddress>,
    ) -> TransferRequiredBadges {
        let os =
            prepare_os(MockNetworkingDriver::new_with_responses(responses))
                .await;
        os.transfer_required_badges(account().address, resources)
            .await
            .unwrap()
    }

    #[actix_rt::test]
    async fn nothing_is_fetched_for_no_resources() {
        let result = required_badges(vec![], vec![]).await;
        assert_eq!(result, TransferRequiredBadges::default());
    }

    #[actix_rt::test]
    async fn freely_withdrawable_resources_need_nothing_more() {
        let result = required_badges(
            vec![withdraw_rules_response([(token(), AccessRule::AllowAll)])],
            vec![token()],
        )
        .await;
        assert_eq!(result, TransferRequiredBadges::default());
    }

    #[actix_rt::test]
    async fn a_restricted_token_gets_a_proof_of_its_badge() {
        let result = required_badges(
            vec![
                withdraw_rules_response([(
                    token(),
                    protected(Requirement::Resource { resource: badge() }),
                )]),
                fungible_vaults_response(3),
            ],
            vec![token()],
        )
        .await;

        assert!(result.is_satisfied());
        assert_eq!(
            result.required_badges,
            vec![ResourceSpecifier::fungible(badge(), Decimal192::one())]
        );
        assert_eq!(
            result.restricted_withdrawals,
            vec![RestrictedWithdrawal::new(
                token(),
                [badge()],
                RestrictedWithdrawalStatus::Satisfied
            )]
        );
    }

    #[actix_rt::test]
    async fn a_missing_badge_is_reported() {
        let result = required_badges(
            vec![
                withdraw_rules_response([(
                    token(),
                    protected(Requirement::Resource { resource: badge() }),
                )]),
                fungible_vaults_response(0),
            ],
            vec![token()],
        )
        .await;

        assert!(!result.is_satisfied());
        assert!(result.required_badges.is_empty());
        assert_eq!(
            result.restricted_withdrawals[0].status,
            RestrictedWithdrawalStatus::MissingBadges
        );
    }

    #[actix_rt::test]
    async fn a_specific_non_fungible_badge_is_looked_up_by_its_location() {
        let id = NonFungibleLocalId::integer(30);
        let rule = protected(non_fungible_requirement(
            NonFungibleGlobalId::new_unchecked(nft_badge(), id.clone()),
        ));
        let location = MockNetworkingDriverResponse::new_success(
            StateNonFungibleLocationResponse {
                ledger_state: LedgerState::sample(),
                resource_address: nft_badge(),
                non_fungible_ids: vec![StateNonFungibleLocationResponseItem {
                    non_fungible_id: id.clone(),
                    is_burned: false,
                    last_updated_at_state_version: 1,
                    owning_vault_address:
                        VaultAddress::sample_mainnet_non_fungible(),
                    owning_vault_parent_ancestor_address: Some(
                        account().address.into(),
                    ),
                    owning_vault_global_ancestor_address: Some(
                        account().address.into(),
                    ),
                }],
            },
        );

        let result = required_badges(
            vec![withdraw_rules_response([(token(), rule)]), location],
            vec![token()],
        )
        .await;

        assert!(result.is_satisfied());
        assert_eq!(
            result.required_badges,
            vec![ResourceSpecifier::non_fungible(nft_badge(), vec![id])]
        );
    }

    #[actix_rt::test]
    async fn the_signature_of_the_account_needs_no_proof() {
        let key = account()
            .security_state
            .clone()
            .into_unsecured()
            .unwrap()
            .transaction_signing
            .public_key
            .public_key;
        let signature: NonFungibleGlobalId = (
            ScryptoNonFungibleGlobalId::from_public_key(
                ScryptoPublicKey::from(key),
            ),
            NetworkID::Mainnet,
        )
            .into();

        // No other request: signing is enough.
        let result = required_badges(
            vec![withdraw_rules_response([(
                token(),
                protected(non_fungible_requirement(signature)),
            )])],
            vec![token()],
        )
        .await;

        assert!(result.is_satisfied());
        assert!(result.required_badges.is_empty());
    }

    #[actix_rt::test]
    async fn the_signatures_of_a_securified_account_are_not_assumed() {
        // Carla is securified: which of her factors sign varies.
        let carla = Account::sample_at(2);
        let key = carla
            .security_state
            .clone()
            .into_securified()
            .unwrap()
            .security_structure
            .matrix_of_factors
            .primary()
            .all_factors()
            .into_iter()
            .next()
            .cloned()
            .map(HierarchicalDeterministicFactorInstance::try_from)
            .unwrap()
            .unwrap()
            .public_key();
        let signature: NonFungibleGlobalId = (
            ScryptoNonFungibleGlobalId::from_public_key(
                ScryptoPublicKey::from(key),
            ),
            NetworkID::Mainnet,
        )
            .into();

        let req = SargonOS::boot_test_with_networking_driver(Arc::new(
            MockNetworkingDriver::new_with_responses(vec![
                withdraw_rules_response([(
                    token(),
                    protected(non_fungible_requirement(signature)),
                )]),
            ]),
        ));
        let os =
            actix_rt::time::timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, req)
                .await
                .unwrap()
                .unwrap();
        os.update_profile_with(|profile| {
            profile.networks.insert(ProfileNetwork::new(
                NetworkID::Mainnet,
                [carla.clone()],
                [],
                AuthorizedDapps::default(),
                ResourcePreferences::default(),
                MFAFactorInstances::default(),
            ));
            profile.factor_sources.extend(FactorSource::sample_all());
            Ok(())
        })
        .await
        .unwrap();

        let result = os
            .transfer_required_badges(carla.address, vec![token()])
            .await
            .unwrap();

        assert_eq!(
            result.restricted_withdrawals[0].status,
            RestrictedWithdrawalStatus::NotWithdrawable
        );
    }

    #[actix_rt::test]
    async fn a_withdraw_rule_that_cannot_be_converted_is_an_error() {
        let rule = AccessRule::Protected {
            access_rule: CompositeRequirement::ProofRule {
                proof_rule: BasicRequirement::CountOf {
                    count: 1000,
                    list: vec![Requirement::Resource { resource: badge() }],
                },
            },
        };
        let os = prepare_os(MockNetworkingDriver::new_with_responses(vec![
            withdraw_rules_response([(token(), rule)]),
        ]))
        .await;
        assert!(os
            .transfer_required_badges(account().address, vec![token()])
            .await
            .is_err());
    }

    #[actix_rt::test]
    async fn a_badge_sent_with_the_restricted_token_makes_a_transfer() {
        let result = required_badges(
            vec![
                withdraw_rules_response([
                    (
                        token(),
                        protected(Requirement::Resource { resource: badge() }),
                    ),
                    (badge(), AccessRule::AllowAll),
                ]),
                fungible_vaults_response(1),
            ],
            vec![token(), badge()],
        )
        .await;
        assert!(result.is_satisfied());

        let recipient = TransferRecipient::AddressOfExternalAccount {
            value: AccountAddress::sample_mainnet_other(),
        };
        let transfer = |resource: ResourceAddress| {
            PerAssetTransfersOfFungibleResource::new(
                PerAssetFungibleResource::new(resource, None),
                [PerAssetFungibleTransfer::new(
                    recipient.clone(),
                    true,
                    Decimal192::one(),
                )],
            )
        };
        let manifest = TransactionManifest::per_asset_transfers(
            PerAssetTransfers::new(
                account().address,
                [transfer(token()), transfer(badge())],
                [],
            ),
            result.required_badges,
        );

        let instructions = manifest.instructions_string();
        let position = |text: &str| instructions.find(text).unwrap();
        assert!(
            position("create_proof_of_amount") < position("POP_FROM_AUTH_ZONE")
        );
        // The badge is withdrawn once its proof is dropped.
        assert!(
            position("POP_FROM_AUTH_ZONE")
                < instructions.rfind("\"withdraw\"").unwrap()
        );
        assert!(manifest
            .summary()
            .unwrap()
            .classification
            .contains(&ManifestClassification::Transfer));
    }
}
