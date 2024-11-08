use crate::prelude::*;

impl Profile {
    pub async fn create_unsaved_entities_with_factor_source_with_derivation_outcome<
        E: IsEntity + Identifiable,
    >(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        count: u16,
        factor_instances_cache_client: &FactorInstancesCacheClient,
        key_derivation_interactors: Arc<dyn KeysDerivationInteractors>,
        get_name: impl Fn(u32) -> DisplayName, // name of entity at index
    ) -> Result<(
        FactorSourceID,
        IdentifiedVecOf<E>,
        FactorInstancesProviderOutcomeForFactor,
    )> {
        let count = count as usize;

        let fsid = factor_source.factor_source_id();

        let outcome =
            VirtualEntityCreatingInstanceProvider::for_many_entity_vecis(
                count,
                E::entity_kind(),
                factor_instances_cache_client,
                Some(self),
                factor_source,
                network_id,
                key_derivation_interactors,
            )
            .await?;

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

        Ok((fsid, entities, outcome))
    }
}
