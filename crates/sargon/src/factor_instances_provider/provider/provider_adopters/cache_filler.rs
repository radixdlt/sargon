use crate::prelude::*;

/// Uses a `FactorInstancesProvider` to fill the cache with instances for a new FactorSource.
pub struct CacheFiller;

impl CacheFiller {
    /// Uses a `FactorInstancesProvider` to fill the `cache` with FactorInstances for a new FactorSource.
    /// Saves FactorInstances into the mutable `cache` parameter and returns a
    /// copy of the instances.
    pub async fn for_new_factor_source(
        cache_client: Arc<FactorInstancesCacheClient>,
        profile: impl Into<Option<Arc<Profile>>>,
        factor_source: FactorSource,
        network_id: NetworkID, // typically mainnet
        interactor: Arc<dyn KeyDerivationInteractor>,
    ) -> Result<FactorInstancesProviderOutcomeForFactor> {
        let provider = FactorInstancesProvider::new(
            network_id,
            IndexSet::just(factor_source.clone()),
            profile,
            cache_client.clone(),
            interactor,
        );

        let quantities_to_derive = DerivationPreset::all()
            .into_iter()
            .map(|preset| {
                (
                    preset,
                    IndexMap::kv(
                        factor_source.id_from_hash(),
                        preset.cache_filling_quantity(),
                    ),
                )
            })
            .collect::<QuantitiesToDerive>();

        let derived = provider
            .derive_more(
                quantities_to_derive,
                DerivationPurpose::pre_deriving_keys(),
            )
            .await?;

        cache_client.insert_all(&derived).await?;

        todo!("migrate me")

        // let derived =
        //     derived.get(&factor_source.id_from_hash()).unwrap().clone();
        // let outcome = InternalFactorInstancesProviderOutcomeForFactor::new(
        //     factor_source.id_from_hash(),
        //     derived.clone(),
        //     FactorInstances::default(),
        //     FactorInstances::default(),
        //     derived,
        // );
        // Ok(outcome.into())
    }
}
