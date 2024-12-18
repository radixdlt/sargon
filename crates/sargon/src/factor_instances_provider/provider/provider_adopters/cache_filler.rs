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
        println!(
            "🎊 CacheFiller - factor_source: {:?} START",
            factor_source.factor_source_id()
        );
        let provider = FactorInstancesProvider::new(
            network_id,
            IndexSet::just(factor_source.clone()),
            profile,
            cache_client.clone(),
            interactor,
        );
        println!(
            "🎊 CacheFiller - factor_source: {:?} PROVIDER CREATED",
            factor_source.factor_source_id()
        );

        let quantities_to_derive = CacheFillingQuantities::for_factor_source(
            factor_source.id_from_hash(),
        );

        println!(
            "🎊 CacheFiller - factor_source: {:?} quantities_to_derive: {:?}",
            factor_source.factor_source_id(),
            quantities_to_derive
        );
        let pdp_pf_derived = provider
            .derive_more(
                quantities_to_derive,
                DerivationPurpose::pre_deriving_keys(),
            )
            .await?;

        println!(
            "🎊 CacheFiller - factor_source: {:?} derived: #{:?}",
            factor_source.factor_source_id(),
            pdp_pf_derived.clone().values().fold(0, |acc, e| acc
                + e.values().fold(0, |xacc, xe| xacc + xe.len()))
        );
        cache_client.insert(&pdp_pf_derived).await?;
        println!(
            "🎊 CacheFiller - factor_source: {:?} INSERTED INTO CACHE",
            factor_source.factor_source_id()
        );

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
