use crate::prelude::*;

#[async_trait::async_trait]
pub trait OsSyncEntitiesStateOnLedger {
    async fn wip_sync_entities_state_on_ledger(
        &self,
    ) -> Result<EntitySyncOutcome>;

    async fn sync_accounts_deleted_on_ledger(&self) -> Result<bool>;
    async fn check_accounts_deleted_on_ledger(
        &self,
        network_id: NetworkID,
        account_addresses: IndexSet<AccountAddress>,
    ) -> Result<IndexMap<AccountAddress, bool>>;
}

// ==================
// Sync Profile Accounts with status on ledger
// ==================
#[async_trait::async_trait]
impl OsSyncEntitiesStateOnLedger for SargonOS {
    /// **WORK IN PROGRESS**
    ///
    /// Syncs entities in profile with on ledger state.
    /// Returns a summary report of the actions performed.
    ///
    /// Checks performed on entities are:
    /// 1. Checks if active accounts on profile are deleted on ledger.
    ///    Action => to mark them as tombstoned
    /// 2. Checks if entities with provisional shield are securified on ledger.
    ///    Action => to mark them as securified
    /// 3. more checks will be developed...
    async fn wip_sync_entities_state_on_ledger(
        &self,
    ) -> Result<EntitySyncOutcome> {
        let mut entities = Vec::<AddressOfAccountOrPersona>::new();
        entities.extend(self.accounts_on_current_network().map(
            |accounts| {
                accounts
                    .iter()
                    .map(|a| AddressOfAccountOrPersona::from(a.address))
                    .collect_vec()
            },
        )?);
        entities.extend(self.personas_on_current_network().map(
            |personas| {
                personas
                    .iter()
                    .map(|p| AddressOfAccountOrPersona::from(p.address))
                    .collect_vec()
            },
        )?);

        let (gateway_client, network_id) = self.gateway_client_on()?;

        // Fetch ancestor addresses
        let badge_owner_per_entity = gateway_client
            .fetch_entities_badge_owners(network_id, entities)
            .await?;

        // Collect sync actions based on the profile state
        let mut sync_actions = IndexSet::<EntitySyncAction>::new();
        for (entity_address, maybe_badge_owner) in badge_owner_per_entity {
            let Some(ancestor_address) = maybe_badge_owner else {
                continue;
            };

            let sync_action = match ancestor_address {
                Address::AccessController(
                    access_controller_ancestor_address,
                ) => {
                    let entity = self.entity_by_address(entity_address)?;

                    if !entity.is_securified() {
                        EntitySyncAction::ToSecurify(
                            entity_address,
                            access_controller_ancestor_address,
                        )
                    } else {
                        // TODO check if it needs to update security state
                        // currently returns no update
                        continue;
                    }
                }
                Address::Account(account_ancestor_address) => {
                    if AddressOfAccountOrPersona::from(account_ancestor_address)
                        == entity_address
                    {
                        EntitySyncAction::ToTombstone(account_ancestor_address)
                    } else {
                        continue;
                    }
                }
                _ => {
                    continue;
                }
            };

            sync_actions.insert(sync_action);
        }

        if !sync_actions.is_empty() {
            // Perform sync
            self.update_profile_with(|profile| {
                let mut actions_performed = IndexSet::<EntitySyncActionPerformed>::new();

                for action in &sync_actions {
                    match action {
                        EntitySyncAction::ToTombstone(account_address) => {
                            profile.networks.tombstone_account(account_address);
                            actions_performed.insert(EntitySyncActionPerformed::SomeEntitiesTombstoned);
                        }
                        EntitySyncAction::ToSecurify(
                            entity_address,
                            access_controller_address
                        ) => {
                            profile.mark_entity_as_securified(
                                *access_controller_address,
                                *entity_address
                            )?;

                            // TODO What if not in provisional state?
                            // 1. Should we not early return error, perform sync in all
                            //    remaining entities and
                            // 2. report that this entity is in bad state?

                            actions_performed.insert(EntitySyncActionPerformed::SomeEntitiesSecurified);
                        }
                    }
                }

                Ok(EntitySyncOutcome::new(actions_performed))
            }).await
        } else {
            Ok(EntitySyncOutcome::no_action())
        }
    }

    /// Checks all active accounts in current network on ledger, if any of them are deleted.
    /// Any deleted account is marked as tombstoned in profile.
    ///
    /// Returns true if any account became tombstoned.
    async fn sync_accounts_deleted_on_ledger(&self) -> Result<bool> {
        let network_id = self.current_network_id()?;
        let accounts = self.accounts_on_current_network()?;

        let account_addresses_with_deleted_status = self
            .check_accounts_deleted_on_ledger(
                network_id,
                accounts.iter().map(|a| a.address).collect(),
            )
            .await?;

        let account_addresses_to_tombstone =
            account_addresses_with_deleted_status
                .iter()
                .filter_map(|(account_address, is_deleted)| {
                    if *is_deleted {
                        Some(*account_address)
                    } else {
                        None
                    }
                })
                .collect_vec();

        let is_any_account_tombstoned =
            !account_addresses_to_tombstone.is_empty();

        if is_any_account_tombstoned {
            self.mark_accounts_as_tombstoned(account_addresses_to_tombstone)
                .await?;
        }

        Ok(is_any_account_tombstoned)
    }

    /// Queries all `account_addresses` on ledger and checks reports which one is deleted.
    ///
    /// Returns an array of the account addresses along with a `bool` being true if that account
    /// is deleted
    async fn check_accounts_deleted_on_ledger(
        &self,
        network_id: NetworkID,
        account_addresses: IndexSet<AccountAddress>,
    ) -> Result<IndexMap<AccountAddress, bool>> {
        let gateway_client = self.gateway_client_with(network_id);

        gateway_client
            .check_accounts_are_deleted(network_id, account_addresses)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt::time::timeout;
    use radix_common::prelude::ACCOUNT_OWNER_BADGE as SCRYPTO_ACCOUNT_OWNER_BADGE;
    use radix_common::prelude::IDENTITY_OWNER_BADGE as SCRYPTO_IDENTITY_OWNER_BADGE;
    use sargon_os_factors::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    impl EntitySyncAction {
        fn entity_address(&self) -> AddressOfAccountOrPersona {
            match self {
                EntitySyncAction::ToTombstone(account_address) => {
                    AddressOfAccountOrPersona::from(*account_address)
                }
                EntitySyncAction::ToSecurify(entity_address, _) => {
                    *entity_address
                }
            }
        }
    }

    #[actix_rt::test]
    async fn test_sync_accounts_deleted_on_ledger() {
        // ARRANGE
        let account_deleted_on_ledger = Account::sample_mainnet_alice();
        let account_active_on_ledger = Account::sample_mainnet_bob();
        let all_initial_accounts = vec![
            account_deleted_on_ledger.clone(),
            account_active_on_ledger.clone(),
        ];
        let mock_driver = MockNetworkingDriver::new_with_responses(vec![
            mock_location_response(vec![account_deleted_on_ledger.address]),
            mock_location_response(vec![account_deleted_on_ledger.address]),
        ]);
        let req = SUT::boot_test_with_networking_driver(Arc::new(mock_driver));
        let os = timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, req)
            .await
            .unwrap()
            .unwrap();
        os.import_wallet(
            &Profile::with(
                Header::sample(),
                FactorSources::sample(),
                AppPreferences::sample(),
                ProfileNetworks::just(ProfileNetwork::new_with_accounts(
                    NetworkID::Mainnet,
                    all_initial_accounts.clone(),
                )),
            ),
            true,
        )
        .await
        .unwrap();

        // ACT
        // Assert first that accounts in profile are active
        assert_eq!(
            os.accounts_on_current_network().unwrap(),
            Accounts::from_iter(all_initial_accounts.clone())
        );
        let result = os.sync_accounts_deleted_on_ledger().await.unwrap();

        // ASSERT
        // An account is deleted
        assert!(result);
        // Only one account is active, the result is persisted in profile
        assert_eq!(
            os.accounts_on_current_network().unwrap(),
            Accounts::from_iter(vec![account_active_on_ledger])
        );
        // Same request does not respond with any accounts deleted
        assert!(!os.sync_accounts_deleted_on_ledger().await.unwrap())
    }

    #[actix_rt::test]
    async fn test_check_accounts_deleted_on_ledger() {
        // ARRANGE
        let account_address_deleted = AccountAddress::sample_stokenet();
        let account_address_not_deleted =
            AccountAddress::sample_stokenet_other();
        let mock_driver = MockNetworkingDriver::new_with_responses(vec![
            mock_location_response(vec![account_address_deleted]),
        ]);
        let req = SUT::boot_test_with_networking_driver(Arc::new(mock_driver));
        let os = timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, req)
            .await
            .unwrap()
            .unwrap();

        // ACT
        let accounts_status = os
            .check_accounts_deleted_on_ledger(
                NetworkID::Stokenet,
                IndexSet::from_iter([
                    account_address_deleted,
                    account_address_not_deleted,
                ]),
            )
            .await
            .unwrap();

        // ASSERT
        assert_eq!(
            accounts_status,
            IndexMap::<AccountAddress, bool>::from_iter([
                (account_address_deleted, true),
                (account_address_not_deleted, false),
            ])
        )
    }

    // Addresses passed to this function, will be considered as deleted by the mocked network driver
    fn mock_location_response(
        deleted_account_addresses: Vec<AccountAddress>,
    ) -> MockNetworkingDriverResponse {
        let body = StateNonFungibleLocationResponse {
            ledger_state: LedgerState::sample_stokenet(),
            resource_address: ResourceAddress::new_from_node_id(
                SCRYPTO_ACCOUNT_OWNER_BADGE,
                NetworkID::Stokenet,
            )
            .unwrap(),
            non_fungible_ids: deleted_account_addresses
                .iter()
                .map(|a| {
                    let local_id = NonFungibleLocalId::from(*a);
                    let parent = Address::from(*a);
                    StateNonFungibleLocationResponseItem {
                        non_fungible_id: local_id,
                        is_burned: false,
                        last_updated_at_state_version: 0,
                        owning_vault_address: VaultAddress::sample_stokenet(),
                        owning_vault_parent_ancestor_address: Some(parent),
                        owning_vault_global_ancestor_address: Some(parent),
                    }
                })
                .collect_vec(),
        };

        MockNetworkingDriverResponse::new_success(body)
    }

    #[actix_rt::test]
    async fn test_sync_entities_on_ledger() {
        // ARRANGE
        let account_deleted_on_ledger = Account::sample_mainnet_alice();
        let account_active_on_ledger = Account::sample_mainnet_bob();
        let account_securified_on_ledger = Account::sample_mainnet_carol();
        let persona_securified_on_ledger = Persona::sample_mainnet();
        let all_initial_accounts = vec![
            account_deleted_on_ledger.clone(),
            account_active_on_ledger.clone(),
            account_securified_on_ledger.clone(),
        ];
        let mock_driver = MockNetworkingDriver::new_with_responses(
            mock_location_responses(vec![
                EntitySyncAction::ToTombstone(
                    account_deleted_on_ledger.address,
                ),
                EntitySyncAction::ToSecurify(
                    AddressOfAccountOrPersona::from(
                        account_securified_on_ledger.address,
                    ),
                    AccessControllerAddress::sample_mainnet(),
                ),
                EntitySyncAction::ToSecurify(
                    AddressOfAccountOrPersona::from(
                        persona_securified_on_ledger.clone().address,
                    ),
                    AccessControllerAddress::sample_mainnet_other(),
                ),
            ]),
        );
        let req = SUT::boot_test_with_networking_driver(Arc::new(mock_driver));
        let sut = timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, req)
            .await
            .unwrap()
            .unwrap();
        sut.import_wallet(
            &Profile::with(
                Header::sample(),
                FactorSources::sample(),
                AppPreferences::sample(),
                ProfileNetworks::just(ProfileNetwork::new(
                    NetworkID::Mainnet,
                    all_initial_accounts.clone(),
                    vec![persona_securified_on_ledger.clone()],
                    AuthorizedDapps::new(),
                    ResourcePreferences::new(),
                    MFAFactorInstances::new(),
                )),
            ),
            true,
        )
        .await
        .unwrap();

        sut.add_factor_sources(FactorSources::sample_values_all_hd())
            .await
            .unwrap();
        let structure_source_ids_sample =
            SecurityStructureOfFactorSourceIDs::sample();
        sut.add_security_structure_of_factor_source_ids(
            &structure_source_ids_sample,
        )
        .await
        .unwrap();
        sut.apply_security_shield_with_id_to_entities(
            structure_source_ids_sample.id(),
            IndexSet::from_iter([
                AddressOfAccountOrPersona::from(
                    account_securified_on_ledger.address,
                ),
                AddressOfAccountOrPersona::from(
                    persona_securified_on_ledger.address,
                ),
            ]),
        )
        .await
        .unwrap();

        let report = sut.wip_sync_entities_state_on_ledger().await.unwrap();

        assert!(report
            .actions_performed
            .contains(&EntitySyncActionPerformed::SomeEntitiesTombstoned));
        assert!(report
            .actions_performed
            .contains(&EntitySyncActionPerformed::SomeEntitiesSecurified));

        let mutated_profile = sut.profile().unwrap();
        let network = mutated_profile.current_network().unwrap();
        assert!(network
            .accounts
            .iter()
            .find(|a| a.address == account_deleted_on_ledger.address())
            .unwrap()
            .is_tombstoned());
        assert!(sut
            .account_by_address(account_securified_on_ledger.address())
            .unwrap()
            .is_securified());
        assert!(sut
            .persona_by_address(persona_securified_on_ledger.address())
            .unwrap()
            .is_securified());
    }

    fn mock_location_responses(
        sync_actions: Vec<EntitySyncAction>,
    ) -> Vec<MockNetworkingDriverResponse> {
        let location_responses = |input: Vec<EntitySyncAction>| -> Vec<
            StateNonFungibleLocationResponseItem,
        > {
            input
                .into_iter()
                .map(|action| {
                    let (ancestor, local_id) = match action {
                        EntitySyncAction::ToTombstone(account_address) => (
                            Address::from(account_address),
                            NonFungibleLocalId::from(account_address),
                        ),
                        EntitySyncAction::ToSecurify(
                            entity_address,
                            access_controller_address,
                        ) => (
                            Address::from(access_controller_address),
                            NonFungibleLocalId::from(entity_address),
                        ),
                    };

                    StateNonFungibleLocationResponseItem {
                        non_fungible_id: local_id,
                        is_burned: false,
                        last_updated_at_state_version: 0,
                        owning_vault_address: VaultAddress::sample_mainnet(),
                        owning_vault_parent_ancestor_address: Some(ancestor),
                        owning_vault_global_ancestor_address: Some(ancestor),
                    }
                })
                .collect_vec()
        };

        let response_for_accounts = MockNetworkingDriverResponse::new_success(
            StateNonFungibleLocationResponse {
                ledger_state: LedgerState::sample(),
                resource_address: ResourceAddress::new_from_node_id(
                    SCRYPTO_ACCOUNT_OWNER_BADGE,
                    NetworkID::Mainnet,
                )
                .unwrap(),
                non_fungible_ids: location_responses(
                    sync_actions
                        .clone()
                        .into_iter()
                        .filter(|action| action.entity_address().is_account())
                        .collect_vec(),
                ),
            },
        );

        let response_for_identities = MockNetworkingDriverResponse::new_success(
            StateNonFungibleLocationResponse {
                ledger_state: LedgerState::sample(),
                resource_address: ResourceAddress::new_from_node_id(
                    SCRYPTO_IDENTITY_OWNER_BADGE,
                    NetworkID::Mainnet,
                )
                .unwrap(),
                non_fungible_ids: location_responses(
                    sync_actions
                        .clone()
                        .into_iter()
                        .filter(|action| action.entity_address().is_identity())
                        .collect_vec(),
                ),
            },
        );

        vec![response_for_accounts, response_for_identities]
    }
}
