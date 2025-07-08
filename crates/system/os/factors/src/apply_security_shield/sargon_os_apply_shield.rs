use crate::prelude::*;

#[async_trait::async_trait]
pub trait OsShieldApplying {
    async fn apply_security_shield_with_id_to_entities(
        &self,
        security_shield_id: SecurityStructureID,
        addresses: IndexSet<AddressOfAccountOrPersona>,
    ) -> Result<EntitiesOnNetwork>;

    async fn _apply_security_structure_of_factor_sources_to_entities_with_diagnostics(
        &self,
        shield: &SecurityStructureOfFactorSources,
        entity_addresses: IndexSet<AddressOfAccountOrPersona>,
    ) -> Result<(
        IdentifiedVecOf<AccountOrPersona>,
        FactorInstancesProviderOutcome,
    )>;

    async fn apply_security_structure_of_factor_sources_to_entities(
        &self,
        shield: &SecurityStructureOfFactorSources,
        entity_addresses: IndexSet<AddressOfAccountOrPersona>,
    ) -> Result<EntitiesOnNetwork>;

    async fn _get_factor_sources_from_security_structure_that_require_spot_check(
        &self,
        shield: &SecurityStructureOfFactorSources,
        entity_addresses: IndexSet<AddressOfAccountOrPersona>,
        network_id: NetworkID,
    ) -> Result<IndexSet<FactorSource>>;

    async fn _perform_spot_check_on_factor_sources(
        &self,
        factor_sources: IndexSet<FactorSource>,
    ) -> Result<()>;

    async fn _provide_instances_using_shield_for_entities_by_address_without_consuming_cache(
        &self,
        security_structure_of_factor_sources: SecurityStructureOfFactorSources, // Aka "shield"
        addresses_of_entities: IndexSet<AddressOfAccountOrPersona>,
    ) -> Result<(
        IndexMap<AddressOfAccountOrPersona, SecurityStructureOfFactorInstances>,
        InstancesInCacheConsumer,
        FactorInstancesProviderOutcome,
    )>;

    /// Without consuming cache, providers instances for the shield for the entities.
    async fn _provide_instances_for_shield_for_entities_without_consuming_cache(
        &self,
        security_structure_of_factor_sources: SecurityStructureOfFactorSources, // Aka "shield"
        entities: IndexSet<AccountOrPersona>,
    ) -> Result<(
        IndexMap<AddressOfAccountOrPersona, SecurityStructureOfFactorInstances>,
        InstancesInCacheConsumer,
        FactorInstancesProviderOutcome,
    )>;
}

#[async_trait::async_trait]
impl OsShieldApplying for SargonOS {
    async fn apply_security_shield_with_id_to_entities(
        &self,
        security_shield_id: SecurityStructureID,
        addresses: IndexSet<AddressOfAccountOrPersona>,
    ) -> Result<EntitiesOnNetwork> {
        let shield = self
            .security_structure_of_factor_sources_from_security_structure_id(
                security_shield_id,
            )?;
        self.apply_security_structure_of_factor_sources_to_entities(
            &shield, addresses,
        )
        .await
    }

    async fn apply_security_structure_of_factor_sources_to_entities(
        &self,
        shield: &SecurityStructureOfFactorSources,
        entity_addresses: IndexSet<AddressOfAccountOrPersona>,
    ) -> Result<EntitiesOnNetwork> {
        let network = self.current_network_id()?;
        let factor_sources = self._get_factor_sources_from_security_structure_that_require_spot_check(shield, entity_addresses.clone(), network).await?;
        self._perform_spot_check_on_factor_sources(factor_sources)
            .await?;
        self._apply_security_structure_of_factor_sources_to_entities_with_diagnostics(
            shield,
            entity_addresses,
        )
        .await
        .and_then(|(entities, _)| EntitiesOnNetwork::new(network, entities))
    }

    async fn _get_factor_sources_from_security_structure_that_require_spot_check(
        &self,
        shield: &SecurityStructureOfFactorSources,
        entity_addresses: IndexSet<AddressOfAccountOrPersona>,
        network_id: NetworkID,
    ) -> Result<IndexSet<FactorSource>> {
        // We only want to perform spot check on Factor Sources for which we won't need to derive public keys.
        // This is, Factor Sources that have enough keys in cache to securify the required entities.
        let mut result: IndexSet<FactorSource> = IndexSet::new();

        // First we get the Accounts & Personas we are going to securify
        let (account_addresses, persona_addresses): (
            Vec<AccountAddress>,
            Vec<IdentityAddress>,
        ) = entity_addresses.iter().partition_map(|a| match a {
            AddressOfAccountOrPersona::Account(account) => {
                Either::Left(*account)
            }
            AddressOfAccountOrPersona::Identity(identity) => {
                Either::Right(*identity)
            }
        });

        let accounts: Accounts = self
            .accounts_on_current_network()?
            .iter()
            .filter(|a| account_addresses.contains(&a.address))
            .collect();
        let personas: Personas = self
            .personas_on_current_network()?
            .iter()
            .filter(|i| persona_addresses.contains(&i.address))
            .collect();

        let cache = self.cache_snapshot().await;

        let roles_factors = shield.matrix_of_factors.all_factors();

        // We iterate over every factor source to find out if we have enough keys in cache, and we need to perform spot check for it.
        for factor_source in shield.all_factors() {
            // To calculate the QuantifiedDerivationPresets, we need to determine how many keys we need for each derivation preset.

            // To determine the entities MFA keys, we check if this factor source is used in any role.
            let account_mfa_count: usize;
            let identity_mfa_count: usize;
            if roles_factors.contains(factor_source) {
                // This factor source is required to securify the entities.
                account_mfa_count = account_addresses.len();
                identity_mfa_count = persona_addresses.len();
            } else {
                // This factor source isn't required to securify the entities (it's only used for ROLA keys).
                account_mfa_count = 0;
                identity_mfa_count = 0;
            }

            // To determine the ROLA keys, we check if this factor source is used as authentication factor.
            let account_rola_count: usize;
            let identity_rola_count: usize;
            if shield.authentication_signing_factor != *factor_source {
                // The factor source isn't used as the shield's authentication signing factor.
                // There are no `AccountRola` or `IdentityRola` derivations keys needed.
                account_rola_count = 0;
                identity_rola_count = 0;
            } else {
                // The factor source is used as authentication factor.
                // We will need to derive ROLA key for each entity that is either unsecurified, or securified with a different factor source.
                fn filter(
                    e: &impl HasSecurityState,
                    factor_source: FactorSource,
                ) -> bool {
                    match e.security_state() {
                        EntitySecurityState::Unsecured { .. } => true,
                        EntitySecurityState::Securified { value: sec } => {
                            sec.security_structure
                                .authentication_signing_factor_instance
                                .factor_source_id
                                != factor_source.id_from_hash()
                        }
                    }
                }

                account_rola_count = accounts
                    .iter()
                    .filter(|a| filter(a, factor_source.clone()))
                    .count();

                identity_rola_count = personas
                    .iter()
                    .filter(|a| filter(a, factor_source.clone()))
                    .count();
            }
            let quantified_derivation_presets: IdentifiedVecOf<
                QuantifiedDerivationPreset,
            > = vec![
                QuantifiedDerivationPreset::new(
                    DerivationPreset::AccountMfa,
                    account_mfa_count,
                ),
                QuantifiedDerivationPreset::new(
                    DerivationPreset::AccountRola,
                    account_rola_count,
                ),
                QuantifiedDerivationPreset::new(
                    DerivationPreset::IdentityMfa,
                    identity_mfa_count,
                ),
                QuantifiedDerivationPreset::new(
                    DerivationPreset::IdentityRola,
                    identity_rola_count,
                ),
            ]
            .into();

            // If cache is satisfied, we won't need to derive public keys so we perform spot check
            if cache.is_satisfied(
                network_id,
                factor_source.id_from_hash(),
                &quantified_derivation_presets,
            ) {
                result.insert(factor_source.clone());
            }
        }

        Ok(result)
    }

    async fn _perform_spot_check_on_factor_sources(
        &self,
        factor_sources: IndexSet<FactorSource>,
    ) -> Result<()> {
        let interactor = self.spot_check_interactor();
        for factor_source in factor_sources {
            let _ = interactor.spot_check(factor_source.clone(), true).await?;
        }
        Ok(())
    }

    async fn _apply_security_structure_of_factor_sources_to_entities_with_diagnostics(
        &self,
        shield: &SecurityStructureOfFactorSources,
        entity_addresses: IndexSet<AddressOfAccountOrPersona>,
    ) -> Result<(
        IdentifiedVecOf<AccountOrPersona>,
        FactorInstancesProviderOutcome,
    )> {
        // TODO change when queue is introduced
        // `CannotSecurifyEntityHasProvisionalSecurityConfig` should be returned when
        // 1. entity's shield is in provisional state and
        // 2. interaction is in the queue.

        // if !entity_addresses
        //     .iter()
        //     .map(|a| self.entity_by_address(*a))
        //     .all(|r| match r {
        //         Ok(e) => e.get_provisional().is_none(),
        //         Err(_) => false,
        //     })
        // {
        //     return Err(
        //         CommonError::CannotSecurifyEntityHasProvisionalSecurityConfig,
        //     );
        // }

        let outcome = self._provide_instances_using_shield_for_entities_by_address_without_consuming_cache(
            shield.clone(),
            entity_addresses.clone().into_iter().map(Into::into).collect()).await?;

        let (
            security_structures_of_factor_instances,
            instances_in_cache_consumer,
            derivation_outcome,
        ) = outcome;

        let mut security_structures_of_factor_instances =
            security_structures_of_factor_instances;

        // consume!
        instances_in_cache_consumer.consume().await?;

        let entities_with_provisional_config = entity_addresses
            .into_iter()
            .map(|entity_address| {
                let security_structure_of_factor_instances =
                    security_structures_of_factor_instances
                        .shift_remove(&entity_address)
                        .expect(
                            "Should have a security structure for each entity",
                        );

                let mut entity = self.entity_by_address(entity_address)?;
                entity.set_provisional(
                    ProvisionalSecurifiedConfig::FactorInstancesDerived {
                        value: security_structure_of_factor_instances,
                    },
                );
                Ok(entity)
            })
            .collect::<Result<IdentifiedVecOf<AccountOrPersona>>>()?;

        assert!(security_structures_of_factor_instances.is_empty());

        // Assert that none of the NEW FactorInstances collide with the existing ones
        self.profile()
            .unwrap()
            .assert_new_factor_instances_not_already_used_erased(
                entities_with_provisional_config.clone(),
            )?;

        self.update_entities_erased(entities_with_provisional_config.clone())
            .await?;

        Ok((
            entities_with_provisional_config.into_iter().collect(),
            derivation_outcome,
        ))
    }

    async fn _provide_instances_using_shield_for_entities_by_address_without_consuming_cache(
        &self,
        security_structure_of_factor_sources: SecurityStructureOfFactorSources, // Aka "shield"
        addresses_of_entities: IndexSet<AddressOfAccountOrPersona>,
    ) -> Result<(
        IndexMap<AddressOfAccountOrPersona, SecurityStructureOfFactorInstances>,
        InstancesInCacheConsumer,
        FactorInstancesProviderOutcome,
    )> {
        let entities = addresses_of_entities
            .iter()
            .map(|a| self.entity_by_address(*a))
            .collect::<Result<IndexSet<AccountOrPersona>>>()?;

        self._provide_instances_for_shield_for_entities_without_consuming_cache(
            security_structure_of_factor_sources,
            entities,
        )
        .await
    }

    async fn _provide_instances_for_shield_for_entities_without_consuming_cache(
        &self,
        security_structure_of_factor_sources: SecurityStructureOfFactorSources, // Aka "shield"
        entities: IndexSet<AccountOrPersona>,
    ) -> Result<(
        IndexMap<AddressOfAccountOrPersona, SecurityStructureOfFactorInstances>,
        InstancesInCacheConsumer,
        FactorInstancesProviderOutcome,
    )> {
        // TODO change when queue is introduced
        // `CannotSecurifyEntityHasProvisionalSecurityConfig` should be returned when
        // 1. entity's shield is in provisional state and
        // 2. interaction is in the queue.

        // if !entities.iter().all(|a| a.get_provisional().is_none()) {
        //     return Err(
        //         CommonError::CannotSecurifyEntityHasProvisionalSecurityConfig,
        //     );
        // }
        let addresses_of_entities = entities
            .iter()
            .map(|e| e.address())
            .collect::<IndexSet<AddressOfAccountOrPersona>>(
        );

        let profile_snapshot = self.profile()?;
        let key_derivation_interactors = self.keys_derivation_interactor();

        // We only consume ROLA factors for:
        // * Unsecured entities (because they do not yet have any ROLA key)
        // * Securified entities where the ROLA key's FactorSource does not match the one of the shield
        let mut existing_rola_key_for_entities = IndexMap::<
            AddressOfAccountOrPersona,
            HierarchicalDeterministicFactorInstance,
        >::new();
        let mut addresses_of_entities_to_derive_rola_key_for =
            IndexSet::<AddressOfAccountOrPersona>::new();

        for entity in entities.iter() {
            match entity.entity_security_state() {
                EntitySecurityState::Unsecured { .. } => {
                    addresses_of_entities_to_derive_rola_key_for
                        .insert(entity.address());
                }
                EntitySecurityState::Securified { value: sec } => {
                    let existing = sec
                        .security_structure
                        .authentication_signing_factor_instance;

                    if existing.factor_source_id
                        == security_structure_of_factor_sources
                            .authentication_signing_factor
                            .id_from_hash()
                    {
                        existing_rola_key_for_entities
                            .insert(entity.address(), existing);
                    } else {
                        addresses_of_entities_to_derive_rola_key_for
                            .insert(entity.address());
                    }
                }
            }
        }
        let derived_any_rola_key_for_any_account =
            !addresses_of_entities_to_derive_rola_key_for
                .iter()
                .filter(|e| e.is_account())
                .collect_vec()
                .is_empty();
        let derived_any_rola_key_for_any_persona =
            !addresses_of_entities_to_derive_rola_key_for
                .iter()
                .filter(|e| e.is_identity())
                .collect_vec()
                .is_empty();

        let (instances_in_cache_consumer, outcome) =
            SecurifyEntityFactorInstancesProvider::apply_security_shield(
                Arc::new(self.factor_instances_cache.clone()),
                Arc::new(profile_snapshot.clone()),
                security_structure_of_factor_sources.clone(),
                addresses_of_entities.clone(),
                addresses_of_entities_to_derive_rola_key_for,
                key_derivation_interactors,
            )
            .await?;

        let mut instances_per_preset_per_factor_source = outcome
            .clone()
            .per_derivation_preset
            .into_iter()
            .map(|(preset, pf)| {
                (
                    preset,
                    pf
                    .per_factor
                    .into_iter()
                    .map(|(k, v)| (k, v.to_use_directly)).collect::<IndexMap<FactorSourceIDFromHash, FactorInstances>>()
                )
            })
            .collect::<InstancesPerDerivationPresetPerFactorSource>();

        assert_eq!(
            instances_per_preset_per_factor_source
                .clone()
                .into_iter()
                .flat_map(|(_, y)| {
                    y.into_iter()
                        .map(|(a, _)| a)
                        .collect::<HashSet<FactorSourceIDFromHash>>()
                })
                .collect::<HashSet<FactorSourceIDFromHash>>(),
            security_structure_of_factor_sources
                .all_factors()
                .into_iter()
                .map(|f| f.id_from_hash())
                .collect::<HashSet<FactorSourceIDFromHash>>()
        );

        let mut security_structures_of_factor_instances = IndexMap::<
            AddressOfAccountOrPersona,
            SecurityStructureOfFactorInstances,
        >::new();

        let mut distribute_instances_for_entity_of_kind_if_needed =
            |entity_kind: CAP26EntityKind,
             derived_any_rola_key: bool|
             -> Result<()> {
                let addresses_of_kind = addresses_of_entities
                    .iter()
                    .filter(|a| a.get_entity_kind() == entity_kind)
                    .collect::<IndexSet<_>>();

                if addresses_of_kind.is_empty() {
                    return Ok(());
                };

                let mut instances_per_factor_source = {
                    let tx_preset =
                        DerivationPreset::mfa_entity_kind(entity_kind);
                    let rola_preset =
                        DerivationPreset::rola_entity_kind(entity_kind);

                    let instances_per_factor_source_mfa = instances_per_preset_per_factor_source
                    .swap_remove(&tx_preset)
                    .unwrap_or_else(|| panic!("Expected to find instances for derivation preset: {:?}", tx_preset));

                    let instances_per_factor_source_rola =
                        if derived_any_rola_key {
                            instances_per_preset_per_factor_source
                        .swap_remove(&rola_preset)
                        .unwrap_or_else(|| panic!("Expected to find instances for derivation preset: {:?}", rola_preset))
                        } else {
                            // No ROLA keys derived, every entity reused existing instances.
                            IndexMap::new()
                        };

                    // Merge `instances_per_factor_source_mfa` and `instances_per_factor_source_rola` together
                    let mut instances_per_factor_source =
                        instances_per_factor_source_mfa;
                    for (k, v) in instances_per_factor_source_rola {
                        instances_per_factor_source.append_or_insert_to(k, v);
                    }
                    instances_per_factor_source
                };

                for entity_address in addresses_of_kind.clone().into_iter() {
                    let security_structure_of_factor_instances = SecurityStructureOfFactorInstances::fulfilling_structure_of_factor_sources_with_instances(
                        &mut instances_per_factor_source,
                        existing_rola_key_for_entities.get(entity_address).cloned(),
                        &security_structure_of_factor_sources
                    )?;

                    security_structures_of_factor_instances.insert(
                        *entity_address,
                        security_structure_of_factor_instances,
                    );
                }

                Ok(())
            };

        distribute_instances_for_entity_of_kind_if_needed(
            CAP26EntityKind::Account,
            derived_any_rola_key_for_any_account,
        )?;
        distribute_instances_for_entity_of_kind_if_needed(
            CAP26EntityKind::Identity,
            derived_any_rola_key_for_any_persona,
        )?;

        Ok((
            security_structures_of_factor_instances,
            instances_in_cache_consumer,
            outcome,
        ))
    }
}

#[cfg(test)]
pub(crate) fn unsafe_shield_with_bdfs(
    bdfs: &FactorSource,
) -> SecurityStructureOfFactorSourceIDs {
    let id = bdfs.factor_source_id();

    // This is an invalid shield, but it's just for testing
    let matrix = unsafe {
        MatrixOfFactorSourceIds::unbuilt_with_roles_and_days(
            PrimaryRoleWithFactorSourceIDs::unbuilt_with_factors(
                Threshold::zero(),
                [],
                [id],
            ),
            RecoveryRoleWithFactorSourceIDs::unbuilt_with_factors(
                Threshold::zero(),
                [],
                [id],
            ),
            ConfirmationRoleWithFactorSourceIDs::unbuilt_with_factors(
                Threshold::zero(),
                [],
                [id],
            ),
            TimePeriod::with_days(14),
        )
    };
    SecurityStructureOfFactorSourceIds::new(
        DisplayName::new("Invalid Shield").unwrap(),
        matrix,
        id,
    )
}

#[cfg(test)]
pub(crate) async fn add_unsafe_shield_with_matrix_with_fixed_metadata(
    os: &SargonOS,
    fixed_metadata: impl Into<Option<SecurityStructureMetadata>>,
) -> Result<SecurityStructureOfFactorSourceIDs> {
    let bdfs = os.bdfs();
    let mut shield_of_ids = unsafe_shield_with_bdfs(&bdfs.into());
    if let Some(fixed_metadata) = fixed_metadata.into() {
        shield_of_ids.metadata = fixed_metadata;
    }
    os.add_security_structure_of_factor_source_ids(&shield_of_ids)
        .await?;
    Ok(shield_of_ids)
}

#[cfg(test)]
pub(crate) async fn add_unsafe_shield_with_matrix(
    os: &SargonOS,
) -> Result<SecurityStructureOfFactorSourceIDs> {
    add_unsafe_shield_with_matrix_with_fixed_metadata(os, None).await
}

#[cfg(test)]
pub(crate) async fn add_unsafe_shield(
    os: &SargonOS,
) -> Result<SecurityStructureID> {
    add_unsafe_shield_with_matrix(os).await.map(|s| s.id())
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {

    use super::*;

    #[actix_rt::test]
    async fn test_apply_security_shield_with_id_to_unsecurified_entities_only()
    {
        // ARRANGE
        let (os, shield_id, account, persona) = {
            let os = SargonOS::fast_boot().await;
            let shield_id = add_unsafe_shield(&os).await.unwrap();
            let network = NetworkID::Mainnet;
            let account = os
                .create_and_save_new_account_with_bdfs(
                    network,
                    DisplayName::sample(),
                )
                .await
                .unwrap();
            let persona = os
                .create_and_save_new_persona_with_bdfs(
                    network,
                    DisplayName::sample_other(),
                    None,
                )
                .await
                .unwrap();
            (os, shield_id, account, persona)
        };

        // ACT
        let (account_provisional, persona_provisional) = {
            os.apply_security_shield_with_id_to_entities(
                shield_id,
                [
                    AddressOfAccountOrPersona::from(account.address()),
                    AddressOfAccountOrPersona::from(persona.address()),
                ]
                .iter()
                .cloned()
                .collect(),
            )
            .await
            .unwrap();
            let account = os.account_by_address(account.address()).unwrap();
            let persona = os.persona_by_address(persona.address()).unwrap();
            let account_provisional = account
                .get_provisional()
                .and_then(|p| p.as_factor_instances_derived().cloned())
                .unwrap();
            let persona_provisional = persona
                .get_provisional()
                .and_then(|p| p.as_factor_instances_derived().cloned())
                .unwrap();
            (account_provisional, persona_provisional)
        };

        // ASSERT
        assert_eq!(account_provisional.security_structure_id, shield_id);
        assert_eq!(persona_provisional.security_structure_id, shield_id);
    }

    #[actix_rt::test]
    async fn test_apply_security_shield_with_id_to_securified_entities_only() {
        // ARRANGE
        let (os, shield_id, account, persona) = {
            let os = SargonOS::fast_boot().await;
            let shield = add_unsafe_shield_with_matrix(&os).await.unwrap();
            let shield_id = shield.id();
            let network = NetworkID::Mainnet;
            let account = os
                .create_and_save_new_account_with_bdfs(
                    network,
                    DisplayName::sample(),
                )
                .await
                .unwrap();
            let persona = os
                .create_and_save_new_persona_with_bdfs(
                    network,
                    DisplayName::sample_other(),
                    None,
                )
                .await
                .unwrap();

            os.apply_security_shield_with_id_to_entities(
                shield_id,
                IndexSet::from_iter([
                    AddressOfAccountOrPersona::from(account.address()),
                    AddressOfAccountOrPersona::from(persona.address()),
                ]),
            )
            .await
            .unwrap();

            // Dummy impl of securifying entities
            let (securified_account, securified_persona) = {
                let mut account =
                    os.account_by_address(account.address()).unwrap();
                let mut persona =
                    os.persona_by_address(persona.address()).unwrap();

                let mut account_security_structure_of_instances = account
                    .get_provisional()
                    .unwrap()
                    .as_factor_instances_derived()
                    .unwrap()
                    .clone();

                // Here we ensure that we test that we reuse the existing ROLA key for the persona below, but not for this account, i.e. the existing ROLA key of this account will mismatch that of the shield.
                account_security_structure_of_instances
                    .authentication_signing_factor_instance =
                    HierarchicalDeterministicFactorInstance::sample_other();
                assert_ne!(
                    FactorSourceID::from(
                        account_security_structure_of_instances
                            .authentication_signing_factor_instance
                            .factor_source_id
                    ),
                    shield.authentication_signing_factor
                );

                let account_secured_control = SecuredEntityControl::new(
                    account
                        .clone()
                        .security_state()
                        .as_unsecured()
                        .unwrap()
                        .transaction_signing
                        .clone(),
                    AccessControllerAddress::sample_mainnet(),
                    account_security_structure_of_instances,
                )
                .unwrap();
                account
                    .set_security_state(EntitySecurityState::Securified {
                        value: account_secured_control,
                    })
                    .unwrap();
                os.update_account(account.clone()).await.unwrap();

                let persona_security_structure_of_instances = persona
                    .get_provisional()
                    .unwrap()
                    .as_factor_instances_derived()
                    .unwrap()
                    .clone();
                let persona_secured_control = SecuredEntityControl::new(
                    persona
                        .clone()
                        .security_state()
                        .as_unsecured()
                        .unwrap()
                        .transaction_signing
                        .clone(),
                    AccessControllerAddress::sample_mainnet_other(),
                    persona_security_structure_of_instances,
                )
                .unwrap();
                persona
                    .set_security_state(EntitySecurityState::Securified {
                        value: persona_secured_control,
                    })
                    .unwrap();
                os.update_persona(persona.clone()).await.unwrap();

                (account, persona)
            };

            (os, shield_id, securified_account, securified_persona)
        };

        // ACT
        let (account_provisional, persona_provisional) = {
            os.apply_security_shield_with_id_to_entities(
                shield_id,
                [
                    AddressOfAccountOrPersona::from(account.address()),
                    AddressOfAccountOrPersona::from(persona.address()),
                ]
                .iter()
                .cloned()
                .collect(),
            )
            .await
            .unwrap();
            let account = os.account_by_address(account.address()).unwrap();
            let persona = os.persona_by_address(persona.address()).unwrap();
            let account_provisional = account
                .get_provisional()
                .and_then(|p| p.as_factor_instances_derived().cloned())
                .unwrap();
            let persona_provisional = persona
                .get_provisional()
                .and_then(|p| p.as_factor_instances_derived().cloned())
                .unwrap();
            (account_provisional, persona_provisional)
        };

        // ASSERT
        assert_eq!(account_provisional.security_structure_id, shield_id);
        assert_eq!(persona_provisional.security_structure_id, shield_id);
        assert_eq!(
            account_provisional
                .matrix_of_factors
                .all_factors()
                .into_iter()
                .filter_map(|f| f.try_as_hd_factor_instances().ok())
                .map(|f| f.derivation_entity_index())
                .collect_vec(),
            persona_provisional
                .matrix_of_factors
                .all_factors()
                .into_iter()
                .filter_map(|f| f.try_as_hd_factor_instances().ok())
                .map(|f| f.derivation_entity_index())
                .collect_vec()
        );
    }

    #[ignore = "Should be tested when queue is integrated"]
    #[actix_rt::test]
    async fn test_one_unsecurified_account_has_provisional_fails() {
        // ARRANGE
        let (os, shield_id, account) = {
            let os = SargonOS::fast_boot().await;
            let shield_id = add_unsafe_shield(&os).await.unwrap();
            let network = NetworkID::Mainnet;
            let mut account = os
                .create_and_save_new_account_with_bdfs(
                    network,
                    DisplayName::sample(),
                )
                .await
                .unwrap();

            account.set_provisional(ProvisionalSecurifiedConfig::sample());

            os.update_account(account.clone()).await.unwrap();
            (os, shield_id, account)
        };

        // ACT
        let result = os
            .apply_security_shield_with_id_to_entities(
                shield_id,
                IndexSet::just(account.address().into()),
            )
            .await;

        // ASSERT
        assert_eq!(
            result,
            Err(CommonError::CannotSecurifyEntityHasProvisionalSecurityConfig)
        );
    }

    #[ignore = "Should be tested when queue is integrated"]
    #[actix_rt::test]
    async fn test_one_securified_account_has_provisional_fails() {
        // ARRANGE
        let (os, shield_id, account) = {
            let os = SargonOS::fast_boot().await;
            let shield_id = add_unsafe_shield(&os).await.unwrap();
            let network = NetworkID::Mainnet;
            let mut account = os
                .create_and_save_new_account_with_bdfs(
                    network,
                    DisplayName::sample(),
                )
                .await
                .unwrap();

            // this is ofc SUPER WRONG! no clue if these factors actually exist in profile...
            account
                .set_security_state(EntitySecurityState::Securified {
                    value: SecuredEntityControl::sample(),
                })
                .unwrap();
            account.set_provisional(ProvisionalSecurifiedConfig::sample());

            os.update_account(account.clone()).await.unwrap();
            (os, shield_id, account)
        };

        // ACT
        let result = os
            .apply_security_shield_with_id_to_entities(
                shield_id,
                IndexSet::just(account.address().into()),
            )
            .await;

        // ASSERT
        assert_eq!(
            result,
            Err(CommonError::CannotSecurifyEntityHasProvisionalSecurityConfig)
        );
    }

    #[ignore = "Should be tested when queue is integrated"]
    #[actix_rt::test]
    async fn test_one_unsecurified_persona_has_provisional_fails() {
        // ARRANGE
        let (os, shield_id, persona) = {
            let os = SargonOS::fast_boot().await;
            let shield_id = add_unsafe_shield(&os).await.unwrap();
            let network = NetworkID::Mainnet;
            let mut persona = os
                .create_and_save_new_persona_with_bdfs(
                    network,
                    DisplayName::sample(),
                    None,
                )
                .await
                .unwrap();

            persona.set_provisional(ProvisionalSecurifiedConfig::sample());

            os.update_persona(persona.clone()).await.unwrap();
            (os, shield_id, persona)
        };

        // ACT
        let result = os
            .apply_security_shield_with_id_to_entities(
                shield_id,
                IndexSet::just(persona.address().into()),
            )
            .await;

        // ASSERT
        assert_eq!(
            result,
            Err(CommonError::CannotSecurifyEntityHasProvisionalSecurityConfig)
        );
    }

    #[ignore = "Should be tested when queue is integrated"]
    #[actix_rt::test]
    async fn test_one_securified_persona_has_provisional_fails() {
        // ARRANGE
        let (os, shield_id, persona) = {
            let os = SargonOS::fast_boot().await;
            let shield_id = add_unsafe_shield(&os).await.unwrap();
            let network = NetworkID::Mainnet;
            let mut persona = os
                .create_and_save_new_persona_with_bdfs(
                    network,
                    DisplayName::sample(),
                    None,
                )
                .await
                .unwrap();

            // this is ofc SUPER WRONG! no clue if these factors actually exist in profile...
            persona
                .set_security_state(EntitySecurityState::Securified {
                    value: SecuredEntityControl::sample_other(),
                })
                .unwrap();
            persona
                .set_provisional(ProvisionalSecurifiedConfig::sample_other());

            os.update_persona(persona.clone()).await.unwrap();
            (os, shield_id, persona)
        };

        // ACT
        let result = os
            .apply_security_shield_with_id_to_entities(
                shield_id,
                IndexSet::just(persona.address().into()),
            )
            .await;

        // ASSERT
        assert_eq!(
            result,
            Err(CommonError::CannotSecurifyEntityHasProvisionalSecurityConfig)
        );
    }

    #[ignore = "Should be tested when queue is integrated"]
    #[actix_rt::test]
    async fn test_one_unsecurified_account_of_many_entities_has_provisional_fails_the_rest_unchanged(
    ) {
        // ARRANGE
        let (os, shield_id, account, personas) = {
            let os = SargonOS::fast_boot().await;
            let shield_id = add_unsafe_shield(&os).await.unwrap();
            let network = NetworkID::Mainnet;
            let mut account = os
                .create_and_save_new_account_with_bdfs(
                    network,
                    DisplayName::sample(),
                )
                .await
                .unwrap();

            account.set_provisional(ProvisionalSecurifiedConfig::sample());

            os.update_account(account.clone()).await.unwrap();

            let personas = os
                .batch_create_many_personas_with_bdfs_then_save_once(
                    3,
                    network,
                    "Persona".to_owned(),
                )
                .await
                .unwrap();

            (os, shield_id, account, personas)
        };

        // ACT
        let mut addresses = personas
            .iter()
            .map(|p| p.address())
            .map(AddressOfAccountOrPersona::from)
            .collect::<IndexSet<_>>();
        addresses.insert(account.address().into());

        let result = os
            .apply_security_shield_with_id_to_entities(shield_id, addresses)
            .await;

        // ASSERT
        assert_eq!(
            result,
            Err(CommonError::CannotSecurifyEntityHasProvisionalSecurityConfig)
        );
        assert_eq!(os.personas_on_current_network().unwrap(), personas); // assert unchanged
    }

    #[ignore = "Should be tested when queue is integrated"]
    #[actix_rt::test]
    async fn test_low_level_one_account_has_provisional_fails() {
        // ARRANGE
        let (os, shield_id, account) = {
            let os = SargonOS::fast_boot().await;
            let shield_id = add_unsafe_shield(&os).await.unwrap();

            let network = NetworkID::Mainnet;
            let mut account = os
                .create_and_save_new_account_with_bdfs(
                    network,
                    DisplayName::sample(),
                )
                .await
                .unwrap();

            account.set_provisional(ProvisionalSecurifiedConfig::sample());

            os.update_account(account.clone()).await.unwrap();
            (os, shield_id, account)
        };

        // ACT
        let shield = os
            .security_structure_of_factor_sources_from_security_structure_id(
                shield_id,
            )
            .unwrap();

        let result = os
            ._provide_instances_for_shield_for_entities_without_consuming_cache(
                shield,
                IndexSet::just(account.into()),
            )
            .await;

        // ASSERT
        assert_eq!(
            result.err().unwrap(),
            CommonError::CannotSecurifyEntityHasProvisionalSecurityConfig
        );
    }

    #[actix_rt::test]
    async fn test_apply_security_shield_continues_when_user_skips_spot_check() {
        // ARRANGE
        let (os, shield_id, account, persona) = {
            let os =
                SargonOS::boot_test_empty_wallet_with_spot_check_interactor(
                    Arc::new(TestSpotCheckInteractor::new_skipped()),
                )
                .await;
            let shield_id = add_unsafe_shield(&os).await.unwrap();
            let network = NetworkID::Mainnet;
            let account = os
                .create_and_save_new_account_with_bdfs(
                    network,
                    DisplayName::sample(),
                )
                .await
                .unwrap();
            let persona = os
                .create_and_save_new_persona_with_bdfs(
                    network,
                    DisplayName::sample_other(),
                    None,
                )
                .await
                .unwrap();
            (os, shield_id, account, persona)
        };

        // ACT
        let (account_provisional, persona_provisional) = {
            os.apply_security_shield_with_id_to_entities(
                shield_id,
                [
                    AddressOfAccountOrPersona::from(account.address()),
                    AddressOfAccountOrPersona::from(persona.address()),
                ]
                .iter()
                .cloned()
                .collect(),
            )
            .await
            .unwrap();
            let account = os.account_by_address(account.address()).unwrap();
            let persona = os.persona_by_address(persona.address()).unwrap();
            let account_provisional = account
                .get_provisional()
                .and_then(|p| p.as_factor_instances_derived().cloned())
                .unwrap();
            let persona_provisional = persona
                .get_provisional()
                .and_then(|p| p.as_factor_instances_derived().cloned())
                .unwrap();
            (account_provisional, persona_provisional)
        };

        // ASSERT
        assert_eq!(account_provisional.security_structure_id, shield_id);
        assert_eq!(persona_provisional.security_structure_id, shield_id);
    }

    #[actix_rt::test]
    async fn spot_check__instances_in_cache_for_all() {
        let (os, shield, entity_addresses, device, ledger, arculus) =
            set_up_spot_check_test().await;

        // Verify that spot check is performed for all Factor Sources (we have keys for all on cache)
        let result = os._get_factor_sources_from_security_structure_that_require_spot_check(&shield, entity_addresses.clone(), NetworkID::Mainnet).await.unwrap();
        let expected: IndexSet<FactorSource> = IndexSet::from_iter([
            device.clone(),
            ledger.clone(),
            arculus.clone(),
        ]);
        pretty_assertions::assert_eq!(result, expected);
    }

    #[actix_rt::test]
    async fn spot_check__instances_for_none() {
        let (os, shield, entity_addresses, _, _, _) =
            set_up_spot_check_test().await;

        // Clear the cache and verify we don't perform spot check for any Factor Source (derivation of keys on both is required)
        os.factor_instances_cache.clear().await.unwrap();
        let result = os._get_factor_sources_from_security_structure_that_require_spot_check(&shield, entity_addresses.clone(), NetworkID::Mainnet).await.unwrap();
        assert!(result.is_empty());
    }

    #[actix_rt::test]
    async fn spot_check__instances_for_one_role_factor_only() {
        let (os, shield, entity_addresses, _, ledger, _) =
            set_up_spot_check_test().await;

        // Set cache to only have keys for `ledger`
        let cache = FactorInstancesCache::build_with_instances(
            ledger.id_from_hash(),
            0,
            2,
            0,
            0,
            1,
            0,
        );
        os.factor_instances_cache
            .set_cache(cache.serializable_snapshot())
            .await
            .unwrap();

        // Verify that spot check is performed for only the ledger Factor Source (we have keys for it on cache, but not for device or arculus)
        let result = os._get_factor_sources_from_security_structure_that_require_spot_check(&shield, entity_addresses.clone(), NetworkID::Mainnet).await.unwrap();
        let expected: IndexSet<FactorSource> =
            IndexSet::from_iter([ledger.clone()]);
        assert_eq!(result, expected);

        // Set similar cache but with one less key, verify spot check isn't performed since we need to derive 1 `AccountMfa` instance
        let cache = FactorInstancesCache::build_with_instances(
            ledger.id_from_hash(),
            0,
            1,
            0,
            0,
            1,
            0,
        );
        os.factor_instances_cache
            .set_cache(cache.serializable_snapshot())
            .await
            .unwrap();

        let result = os._get_factor_sources_from_security_structure_that_require_spot_check(&shield, entity_addresses.clone(), NetworkID::Mainnet).await.unwrap();
        assert!(result.is_empty());
    }

    #[actix_rt::test]
    async fn spot_check__instances_for_rola_factor_only() {
        let (os, shield, entity_addresses, _, _, arculus) =
            set_up_spot_check_test().await;

        // Set cache to have only 1 `AccountRola` key for `arculus`
        let cache = FactorInstancesCache::build_with_instances(
            arculus.id_from_hash(),
            0,
            0,
            1,
            0,
            0,
            1,
        );
        os.factor_instances_cache
            .set_cache(cache.serializable_snapshot())
            .await
            .unwrap();

        // Verify we don't perform spot check for any Factor Source (we need 1 more `AccountRola` key for Arculus).
        let result = os._get_factor_sources_from_security_structure_that_require_spot_check(&shield, entity_addresses.clone(), NetworkID::Mainnet).await.unwrap();
        assert!(result.is_empty());

        // Update one of the accounts and set is as securified with the Arculus as `authentication_signing_factor_instance`
        security_first_account(&os, arculus.id_from_hash())
            .await
            .unwrap();

        // Perform the same check again and verify spot check is performed for Arculus, since we only need 1 `AccountRola` key
        let result = os._get_factor_sources_from_security_structure_that_require_spot_check(&shield, entity_addresses.clone(), NetworkID::Mainnet).await.unwrap();
        assert_eq!(result, IndexSet::just(arculus));
    }

    /// Sets up SargonOS with 3 factor sources: `device`, `ledger` and `arculus`
    /// Creates a Shield that uses `device` and `ledger` for P, R and C roles, and `arculus` for ROLA.
    /// Finally, add 2 Accounts & 1 Persona to the Profile
    async fn set_up_spot_check_test() -> (
        Arc<SargonOS>,
        SecurityStructureOfFactorSources,
        IndexSet<AddressOfAccountOrPersona>,
        FactorSource,
        FactorSource,
        FactorSource,
    ) {
        // Set up OS with 3 factor sources:
        // - `device`: a BDFS with keys in cache
        // - `ledger`: a ledger with keys in cache
        // - `arculus`: an arculusCard with keys in cache
        let os = SargonOS::fast_boot().await;
        let device: FactorSource = os.bdfs().into();
        let ledger = FactorSource::sample_ledger();
        let arculus = FactorSource::sample_arculus();

        os.add_factor_source(ledger.clone()).await.unwrap(); // This pre-derives keys for Ledger as well
        os.add_factor_source(arculus.clone()).await.unwrap(); // This pre-derives keys for Arculus as well

        // Set up a shield that requires all factor sources:
        // `device` & `ledger` are used on P, R and C roles
        // `arculus` is used for ROLA
        let shield = add_sample_shield(
            &os,
            device.factor_source_id(),
            ledger.factor_source_id(),
            arculus.factor_source_id(),
        )
        .await;

        // Add 2 Accounts & 1 Persona to the Profile
        let entity_addresses = add_entities(&os).await;

        (os, shield, entity_addresses, device, ledger, arculus)
    }

    /// Creates and adds 2 Accounts & 1 Persona to the OS
    /// Returns the addresses of the created entities.
    async fn add_entities(
        os: &SargonOS,
    ) -> IndexSet<AddressOfAccountOrPersona> {
        let account1: AddressOfAccountOrPersona = os
            .create_and_save_new_account_with_bdfs(
                NetworkID::Mainnet,
                DisplayName::sample(),
            )
            .await
            .unwrap()
            .address
            .into();

        let account2: AddressOfAccountOrPersona = os
            .create_and_save_new_account_with_bdfs(
                NetworkID::Mainnet,
                DisplayName::sample_other(),
            )
            .await
            .unwrap()
            .address
            .into();

        let persona: AddressOfAccountOrPersona = os
            .create_and_save_new_persona_with_bdfs(
                NetworkID::Mainnet,
                DisplayName::sample_other(),
                None,
            )
            .await
            .unwrap()
            .address
            .into();

        IndexSet::from_iter([account1, account2, persona])
    }

    /// Creates and adds a sample shield with the given device, ledger and arculus factor source ids.
    /// It uses the device and ledger for the primary, recovery and confirmation roles, and arculus as ROLA signing factor.
    async fn add_sample_shield(
        os: &Arc<SargonOS>,
        device_id: FactorSourceID,
        ledger_id: FactorSourceID,
        arculus_id: FactorSourceID,
    ) -> SecurityStructureOfFactorSources {
        let matrix = unsafe {
            MatrixOfFactorSourceIds::unbuilt_with_roles_and_days(
                PrimaryRoleWithFactorSourceIDs::unbuilt_with_factors(
                    Threshold::zero(),
                    [device_id, ledger_id],
                    [],
                ),
                RecoveryRoleWithFactorSourceIDs::unbuilt_with_factors(
                    Threshold::zero(),
                    [],
                    [device_id],
                ),
                ConfirmationRoleWithFactorSourceIDs::unbuilt_with_factors(
                    Threshold::zero(),
                    [],
                    [ledger_id],
                ),
                TimePeriod::with_days(14),
            )
        };
        // Create SEC of FSIds and add it
        let sec = SecurityStructureOfFactorSourceIds::new(
            DisplayName::new("Invalid Shield").unwrap(),
            matrix,
            arculus_id,
        );
        os.add_security_structure_of_factor_source_ids(&sec)
            .await
            .unwrap();

        // Get Shield

        os.security_structure_of_factor_sources_from_security_structure_id(
            sec.id(),
        )
        .unwrap()
    }

    /// Fetches the first Account on Profile and updates it to be securified with an instance of the
    /// given `factor_source_id` as authentication signing factor
    async fn security_first_account(
        os: &SargonOS,
        factor_source_id: FactorSourceIDFromHash,
    ) -> Result<()> {
        let accounts = os.accounts_on_current_network()?;
        let account = accounts.first().unwrap();
        let authentication_siging_instance = HierarchicalDeterministicFactorInstance::new_for_entity_with_key_kind_on_network(
            CAP26KeyKind::AuthenticationSigning,
            NetworkID::Mainnet,
            factor_source_id,
            CAP26EntityKind::Account,
            Hardened::from_local_key_space(
                10u32,
                IsSecurified(true),
            )
                .unwrap(),
        );
        let control = SecuredEntityControl::new(
            account
                .security_state()
                .as_unsecured()
                .unwrap()
                .transaction_signing
                .clone(),
            AccessControllerAddress::sample_mainnet(),
            SecurityStructureOfFactorInstances::new(
                SecurityStructureID::sample(),
                MatrixOfFactorInstances::sample(),
                authentication_siging_instance,
            )?,
        )?;

        let mut securified_account = account.clone();
        securified_account.set_security_state(
            EntitySecurityState::Securified { value: control },
        )?;

        os.update_account(securified_account.clone()).await
    }
}
