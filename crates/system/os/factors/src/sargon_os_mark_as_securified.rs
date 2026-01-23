use crate::prelude::*;

#[async_trait::async_trait]
pub trait OsMarkAsSecurified {
    /// Marks the entities as securified by finding the `AccessControllerAddress` on ledger
    /// and updates the profile.
    async fn mark_entities_as_securified(
        &self,
        entity_addresses: IndexSet<AddressOfAccountOrPersona>,
    ) -> Result<()>;
}

#[async_trait::async_trait]
impl OsMarkAsSecurified for SargonOS {
    async fn mark_entities_as_securified(
        &self,
        entity_addresses: IndexSet<AddressOfAccountOrPersona>,
    ) -> Result<()> {
        let (gateway_client, network_id) = self.gateway_client_on()?;

        // Fetch ancestor addresses
        let badge_owner_per_entity = gateway_client
            .fetch_entities_badge_owners(network_id, entity_addresses.clone())
            .await?;

        self.update_profile_with(|profile| {
            for entity_address in &entity_addresses {
                let maybe_badge_owner = badge_owner_per_entity
                    .get(entity_address)
                    .unwrap_or(&None);

                let Some(access_controller_address) = maybe_badge_owner.and_then(|a| {
                    a.as_access_controller().cloned()
                }) else {
                    return Err(CommonError::EntityIsNotControlledByAnAccessControllerOnLedger {
                        entity_bech32m_encoded_address: entity_address.to_string(),
                    })
                };

                profile.mark_entity_as_securified(
                    access_controller_address,
                    *entity_address,
                )?;
            }

            Ok(())
        }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt::time::timeout;
    use radix_common::prelude::ACCOUNT_OWNER_BADGE as SCRYPTO_ACCOUNT_OWNER_BADGE;
    use radix_common::prelude::IDENTITY_OWNER_BADGE as SCRYPTO_IDENTITY_OWNER_BADGE;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn test_unsecurified_entities_mark_as_securified() {
        // ARRANGE
        let account_securified_on_ledger = Account::sample_mainnet_carol();
        let access_controller_for_account =
            AccessControllerAddress::sample_mainnet();
        let persona_securified_on_ledger = Persona::sample_mainnet();
        let access_controller_for_persona =
            AccessControllerAddress::sample_mainnet_other();
        let mock_driver = MockNetworkingDriver::new_with_responses(
            mock_location_responses(HashMap::from([
                (
                    AddressOfAccountOrPersona::from(
                        account_securified_on_ledger.clone().address,
                    ),
                    Some(Address::from(access_controller_for_account)),
                ),
                (
                    AddressOfAccountOrPersona::from(
                        persona_securified_on_ledger.clone().address,
                    ),
                    Some(Address::from(access_controller_for_persona)),
                ),
            ])),
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
                    vec![account_securified_on_ledger.clone()],
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

        sut.mark_entities_as_securified(IndexSet::from_iter([
            AddressOfAccountOrPersona::from(
                account_securified_on_ledger.address,
            ),
            AddressOfAccountOrPersona::from(
                persona_securified_on_ledger.address,
            ),
        ]))
        .await
        .unwrap();

        assert!(sut
            .account_by_address(account_securified_on_ledger.address())
            .unwrap()
            .is_securified());
        assert!(sut
            .persona_by_address(persona_securified_on_ledger.address())
            .unwrap()
            .is_securified());
    }

    #[actix_rt::test]
    async fn test_unsecurified_account_fails_no_access_controller_on_ledger() {
        // ARRANGE
        let account_not_securified_on_ledger = Account::sample_mainnet_carol();
        let mock_driver = MockNetworkingDriver::new_with_responses(
            mock_location_responses(HashMap::from([(
                AddressOfAccountOrPersona::from(
                    account_not_securified_on_ledger.clone().address,
                ),
                None,
            )])),
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
                    vec![account_not_securified_on_ledger.clone()],
                    vec![],
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
            IndexSet::from_iter([AddressOfAccountOrPersona::from(
                account_not_securified_on_ledger.address,
            )]),
        )
        .await
        .unwrap();

        let result = sut
            .mark_entities_as_securified(IndexSet::from_iter([
                AddressOfAccountOrPersona::from(
                    account_not_securified_on_ledger.address,
                ),
            ]))
            .await;

        assert_eq!(
            Err(CommonError::EntityIsNotControlledByAnAccessControllerOnLedger {
                entity_bech32m_encoded_address: account_not_securified_on_ledger.address.to_string(),
            }),
            result
        );
        assert!(!sut
            .account_by_address(account_not_securified_on_ledger.address())
            .unwrap()
            .is_securified());
    }

    #[actix_rt::test]
    async fn test_unsecurified_account_fails_no_provisional_state() {
        // ARRANGE
        let account_securified_on_ledger = Account::sample_mainnet_carol();
        let access_controller_for_account =
            AccessControllerAddress::sample_mainnet();
        let mock_driver = MockNetworkingDriver::new_with_responses(
            mock_location_responses(HashMap::from([(
                AddressOfAccountOrPersona::from(
                    account_securified_on_ledger.clone().address,
                ),
                Some(Address::from(access_controller_for_account)),
            )])),
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
                    vec![account_securified_on_ledger.clone()],
                    vec![],
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

        let result = sut
            .mark_entities_as_securified(IndexSet::from_iter([
                AddressOfAccountOrPersona::from(
                    account_securified_on_ledger.address,
                ),
            ]))
            .await;

        assert_eq!(
            Err(CommonError::EntityHasNoProvisionalSecurityConfigSet),
            result
        );
        assert!(!sut
            .account_by_address(account_securified_on_ledger.address())
            .unwrap()
            .is_securified());
    }

    #[actix_rt::test]
    async fn test_already_securified_account_fails() {
        // ARRANGE
        let mut account_securified_on_ledger = Account::sample_mainnet_carol();
        let secured_entity_control = SecuredEntityControl::sample();
        let access_controller_for_account =
            secured_entity_control.access_controller_address;
        let mock_driver = MockNetworkingDriver::new_with_responses(
            mock_location_responses(HashMap::from([(
                AddressOfAccountOrPersona::from(
                    account_securified_on_ledger.clone().address,
                ),
                Some(Address::from(access_controller_for_account)),
            )])),
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
                    vec![account_securified_on_ledger.clone()],
                    vec![],
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

        account_securified_on_ledger
            .set_security_state(EntitySecurityState::Securified {
                value: secured_entity_control,
            })
            .unwrap();
        sut.update_account(account_securified_on_ledger.clone())
            .await
            .unwrap();

        let result = sut
            .mark_entities_as_securified(IndexSet::from_iter([
                AddressOfAccountOrPersona::from(
                    account_securified_on_ledger.address,
                ),
            ]))
            .await;

        assert_eq!(
            Err(CommonError::SecurityStateSecurifiedButExpectedUnsecurified),
            result
        );
    }

    fn mock_location_responses(
        entity_with_ancestor: HashMap<
            AddressOfAccountOrPersona,
            Option<Address>,
        >,
    ) -> Vec<MockNetworkingDriverResponse> {
        let location_responses = |input: HashMap<
            AddressOfAccountOrPersona,
            Option<Address>,
        >|
         -> Vec<
            StateNonFungibleLocationResponseItem,
        > {
            input
                .into_iter()
                .map(|(entity, ancestor)| {
                    let local_id = NonFungibleLocalId::from(entity);

                    StateNonFungibleLocationResponseItem {
                        non_fungible_id: local_id,
                        is_burned: false,
                        last_updated_at_state_version: 0,
                        owning_vault_address: VaultAddress::sample_mainnet(),
                        owning_vault_parent_ancestor_address: ancestor,
                        owning_vault_global_ancestor_address: ancestor,
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
                    entity_with_ancestor
                        .clone()
                        .into_iter()
                        .filter(|(entity, _)| entity.is_account())
                        .collect(),
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
                    entity_with_ancestor
                        .clone()
                        .into_iter()
                        .filter(|(entity, _)| entity.is_identity())
                        .collect(),
                ),
            },
        );

        vec![response_for_accounts, response_for_identities]
    }
}
