use crate::prelude::*;

/// Uses a `FactorInstancesProvider` to fill the cache with instances for a new FactorSource.
pub struct CacheFiller;

pub struct CacheFillingQuantities;
impl CacheFillingQuantities {
    pub fn for_factor_source(id: FactorSourceIDFromHash) -> QuantitiesToDerive {
        DerivationPreset::all()
            .into_iter()
            .map(|preset| {
                (preset, IndexMap::kv(id, preset.cache_filling_quantity()))
            })
            .collect::<QuantitiesToDerive>()
    }
}

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
    ) -> Result<FactorInstancesProviderOutcome> {
        let provider = FactorInstancesProvider::new(
            network_id,
            IndexSet::just(factor_source.clone()),
            profile,
            cache_client.clone(),
            interactor,
        );

        let quantities_to_derive = CacheFillingQuantities::for_factor_source(
            factor_source.id_from_hash(),
        );

        let pdp_pf_derived = provider
            .derive_more(
                quantities_to_derive,
                DerivationPurpose::pre_deriving_keys(),
            )
            .await?;

        cache_client.insert(&pdp_pf_derived).await?;

        let per_derivation_preset = pdp_pf_derived
            .into_iter()
            .map(|(preset, v)| {
                let per_factor = v
                    .into_iter()
                    .map(|(factor_source_id, derived)| {
                        assert_eq!(factor_source_id, factor_source.id_from_hash());

                        let internal_for_factor = InternalFactorInstancesProviderOutcomeForFactor::new(
                            factor_source.id_from_hash(),
                            derived.clone(),
                            FactorInstances::default(),
                            FactorInstances::default(),
                            derived,
                        );
                        let for_factor = FactorInstancesProviderOutcomeForFactor::from(internal_for_factor);

                        (factor_source_id, for_factor)
                    })
                    .collect::<IndexMap<
                    FactorSourceIDFromHash,
                    FactorInstancesProviderOutcomeForFactor,
                >>();


                    (preset, FactorInstancesProviderOutcomePerFactor {
                        per_factor,
                    })
            }).collect::<IndexMap<DerivationPreset, FactorInstancesProviderOutcomePerFactor>>();

        Ok(FactorInstancesProviderOutcome {
            per_derivation_preset,
        })
    }
}
