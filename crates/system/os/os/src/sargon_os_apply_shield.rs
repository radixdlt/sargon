use crate::prelude::*;

#[async_trait::async_trait]
pub trait OsShieldApplying {
    async fn apply_security_shield_to_entities(
        &self,
        security_shield_id: SecurityStructureID,
        addresses: IndexSet<AddressOfAccountOrPersona>,
    ) -> Result<()> {
        todo!()
    }

    async fn _apply_shield_to_entities_with_diagnostics(
        &self,
        shield: &SecurityStructureOfFactorSources,
        entity_addresses: IndexSet<AddressOfAccountOrPersona>,
    ) -> Result<(
        IdentifiedVecOf<AccountOrPersona>,
        FactorInstancesProviderOutcome,
    )>;

    async fn _provider_instances_for_shield_for_entities_by_address_without_consuming_cache(
        &self,
        security_structure_of_factor_sources: SecurityStructureOfFactorSources, // Aka "shield"
        addresses_of_entities: IndexSet<AddressOfAccountOrPersona>,
    ) -> Result<(
        IndexMap<AddressOfAccountOrPersona, SecurityStructureOfFactorInstances>,
        InstancesInCacheConsumer,
        FactorInstancesProviderOutcome,
    )>;

    /// Without consuming cache, providers instances for the shield for the entities.
    async fn _provider_instances_for_shield_for_entities_without_consuming_cache(
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
    async fn _apply_shield_to_entities_with_diagnostics(
        &self,
        shield: &SecurityStructureOfFactorSources,
        entity_addresses: IndexSet<AddressOfAccountOrPersona>,
    ) -> Result<(
        IdentifiedVecOf<AccountOrPersona>,
        FactorInstancesProviderOutcome,
    )> {
        if !entity_addresses
            .iter()
            .map(|a| self.entity_by_address(*a))
            .all(|r| match r {
                Ok(e) => e.get_provisional().is_none(),
                Err(_) => false,
            })
        {
            return Err(
                CommonError::CannotSecurifyEntityHasProvisionalSecurityConfig,
            );
        }

        let outcome = self._provider_instances_for_shield_for_entities_by_address_without_consuming_cache(
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

        let securified_entities = entity_addresses
            .into_iter()
            .map(|entity_address| {
                let security_structure_of_factor_instances =
                    security_structures_of_factor_instances
                        .shift_remove(&entity_address)
                        .unwrap();

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
                securified_entities.clone(),
            )?;

        self.update_entities_erased(securified_entities.clone())
            .await?;

        Ok((
            securified_entities.into_iter().collect(),
            derivation_outcome,
        ))
    }

    async fn _provider_instances_for_shield_for_entities_by_address_without_consuming_cache(
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

        self._provider_instances_for_shield_for_entities_without_consuming_cache(
            security_structure_of_factor_sources,
            entities,
        )
        .await
    }

    async fn _provider_instances_for_shield_for_entities_without_consuming_cache(
        &self,
        security_structure_of_factor_sources: SecurityStructureOfFactorSources, // Aka "shield"
        entities: IndexSet<AccountOrPersona>,
    ) -> Result<(
        IndexMap<AddressOfAccountOrPersona, SecurityStructureOfFactorInstances>,
        InstancesInCacheConsumer,
        FactorInstancesProviderOutcome,
    )> {
        if !entities.iter().all(|a| a.get_provisional().is_none()) {
            return Err(
                CommonError::CannotSecurifyEntityHasProvisionalSecurityConfig,
            );
        }
        let addresses_of_entities = entities
            .iter()
            .map(|e| e.address())
            .collect::<IndexSet<AddressOfAccountOrPersona>>(
        );

        let profile_snapshot = self.profile()?;
        let key_derivation_interactors = self.keys_derivation_interactor();

        // We only consume ROLA factors for:
        // * Unsecured entities (because they do not yet have nay ROLA key)
        // * Securified entities where the ROLA key's FactorSource does not match the one of the shield
        let mut existing_rola_key_for_entities =
            IndexMap::<AddressOfAccountOrPersona, HierarchicalDeterministicFactorInstance>::new();
            let mut include_rola_key_for_entities =
            IndexSet::<AddressOfAccountOrPersona>::new();

        for entity in entities.iter() {
            match entity.entity_security_state() {
                EntitySecurityState::Unsecured { .. } => { include_rola_key_for_entities.insert(entity.address()); },
                EntitySecurityState::Securified { value: sec } => {
                    let existing =   sec
                        .security_structure
                        .authentication_signing_factor_instance;
                     
                      if existing
                       .factor_source_id
                        == security_structure_of_factor_sources
                            .authentication_signing_factor
                            .id_from_hash()
                    {
                        existing_rola_key_for_entities.insert(entity.address(), existing);
                    } else {
                        include_rola_key_for_entities.insert(entity.address());
                    }
                }
            }
        }

        let (instances_in_cache_consumer, outcome) =
            SecurifyEntityFactorInstancesProvider::apply_security_shield(
                Arc::new(self.clients.factor_instances_cache.clone()),
                Arc::new(profile_snapshot.clone()),
                security_structure_of_factor_sources.clone(),
                addresses_of_entities.clone(),
                include_rola_key_for_entities,
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
            |entity_kind: CAP26EntityKind| -> Result<()> {
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

                    let instances_per_factor_source_rola = instances_per_preset_per_factor_source
                    .swap_remove(&rola_preset)
                    .unwrap_or_else(|| panic!("Expected to find instances for derivation preset: {:?}", rola_preset));

                    // Merge `instances_per_factor_source_mfa` and `instances_per_factor_source_rola` together
                    let mut instances_per_factor_source =
                        instances_per_factor_source_mfa;
                    for (k, v) in instances_per_factor_source_rola {
                        instances_per_factor_source.append_or_insert_to(k, v);
                    }
                    instances_per_factor_source
                };

                for entity_address in addresses_of_kind.clone().into_iter() {
       
                    let security_structure_of_factor_instances = 
                    SecurityStructureOfFactorInstances::fulfilling_structure_of_factor_sources_with_instances(
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
        )?;
        distribute_instances_for_entity_of_kind_if_needed(
            CAP26EntityKind::Identity,
        )?;

        Ok((
            security_structures_of_factor_instances,
            instances_in_cache_consumer,
            outcome,
        ))
    }
}
