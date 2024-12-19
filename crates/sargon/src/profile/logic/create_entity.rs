use crate::prelude::*;

impl Profile {
    /// Creates `count` many new virtual entities of type `E` on `network_id` with `factor_source` as the factor source.
    /// Setting the names according to `get_name`, loading pre-derived FactorInstances from the
    /// FactorInstancesCache if possible, else derives more using the FactorInstancesProvider.
    ///
    /// Returns the FactorSourceID, the entities, the InstancesInCacheConsumer, and the FactorInstancesProviderOutcomeForFactor.
    ///
    /// The `FactorInstancesProviderOutcomeForFactor` is primarily useful for testing.
    ///
    /// The `InstancesInCacheConsumer` SHOULD be called by the caller, once you know it
    /// is safe to delete the instances from the cache - e.g. after having saved the new
    /// entities into the Profile and persisted it into SecureStorage.
    pub async fn create_unsaved_entities_with_factor_source_with_derivation_outcome<
        E: IsEntity + Identifiable,
    >(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        count: u16,
        factor_instances_cache_client: Arc<FactorInstancesCacheClient>,
        key_derivation_interactor: Arc<dyn KeyDerivationInteractor>,
        get_name: impl Fn(u32) -> DisplayName, // name of entity at index
    ) -> Result<(
        FactorSourceID,
        IdentifiedVecOf<E>,
        InstancesInCacheConsumer,
        FactorInstancesProviderOutcomeForFactor,
    )> {
        let count = count as usize;

        let fsid = factor_source.factor_source_id();
        let entity_kind = E::entity_kind();

        let (instances_in_cache_consumer, outcome) =
            VirtualEntityCreatingInstanceProvider::for_many_entity_vecis(
                count,
                entity_kind,
                factor_instances_cache_client,
                Arc::new(self.clone()),
                factor_source.clone(),
                network_id,
                key_derivation_interactor,
            )
            .await?;

        let outcome = outcome
            .per_derivation_preset
            .get(&DerivationPreset::veci_entity_kind(entity_kind))
            .unwrap()
            .per_factor
            .get(&factor_source.id_from_hash())
            .cloned()
            .unwrap();

        let instances_to_use_directly = outcome.clone().to_use_directly;

        assert_eq!(instances_to_use_directly.len(), count);

        let entities = instances_to_use_directly
            .into_iter()
            .map(|f| {
                HDFactorInstanceTransactionSigning::<E::Path>::new(f).unwrap()
            })
            .map(|veci| {
                let idx = u32::from(
                    veci.path
                        .derivation_path()
                        .index()
                        .index_in_local_key_space(),
                );
                let name = get_name(idx);

                E::with_veci_and_name(veci, name)
            })
            .collect::<IdentifiedVecOf<E>>();

        Ok((fsid, entities, instances_in_cache_consumer, outcome))
    }
}
