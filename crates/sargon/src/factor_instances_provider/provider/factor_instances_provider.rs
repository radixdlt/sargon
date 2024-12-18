use crate::prelude::*;
use crate::{factor_instances_provider::next_index_assigner, prelude::*};

/// A coordinator between a cache, an optional profile and the KeysCollector.
///
/// We can ask this type to provide FactorInstances for some operation, either
/// creation of new virtual accounts or securifying accounts (or analogously for identities).
/// It will try to read instances from the cache, if any, and if there are not enough instances
/// in the cache, it will derive more instances and save them into the cache.
///
/// We are always reading from the beginning of each FactorInstance collection in the cache,
/// and we are always appending to the end.
///
/// Whenever we need to derive more, we always derive for all `IndexAgnosticPath` "presets",
/// i.e. we are not only filling the cache with factor instances relevant to the operation
/// but rather we are filling the cache with factor instances for all kinds of operations, i.e.
/// if we did not have `CACHE_FILLING_QUANTITY` instances for "account_mfa", when we tried
/// to read "account_veci" instances, we will derive more "account_mfa" instances as well,
/// so many that at the end of execution we will have `CACHE_FILLING_QUANTITY` instances for
/// both "account_veci" and "account_mfa" (and same for identities).
pub struct FactorInstancesProvider {
    network_id: NetworkID,
    factor_sources: IndexSet<FactorSource>,
    profile: Option<Arc<Profile>>,
    cache_client: Arc<FactorInstancesCacheClient>,
    interactor: Arc<dyn KeyDerivationInteractor>,
}

/// ===============
/// PUBLIC
/// ===============
impl FactorInstancesProvider {
    pub fn new(
        network_id: NetworkID,
        factor_sources: IndexSet<FactorSource>,
        profile: impl Into<Option<Arc<Profile>>>,
        cache_client: Arc<FactorInstancesCacheClient>,
        interactor: Arc<dyn KeyDerivationInteractor>,
    ) -> Self {
        Self {
            network_id,
            factor_sources,
            profile: profile.into(),
            cache_client,
            interactor,
        }
    }

    pub async fn provide(
        self,
        quantified_derivation_preset: QuantifiedDerivationPreset,
        derivation_purpose: DerivationPurpose,
    ) -> Result<(
        InstancesInCacheConsumer,
        InternalFactorInstancesProviderOutcome,
    )> {
        self.provide_for_presets(
            IdentifiedVecOf::just(quantified_derivation_preset),
            derivation_purpose,
        )
        .await
    }

    pub async fn provide_for_presets(
        self,
        quantified_derivation_presets: IdentifiedVecOf<
            QuantifiedDerivationPreset,
        >,
        derivation_purpose: DerivationPurpose,
    ) -> Result<(
        InstancesInCacheConsumer,
        InternalFactorInstancesProviderOutcome,
    )> {
        let mut _self = self;

        _self
            ._provide_for_presets(
                quantified_derivation_presets,
                derivation_purpose,
            )
            .await
    }
}

/// ===============
/// Private
/// ===============
impl FactorInstancesProvider {
    fn make_instances_in_cache_consumer(
        &self,
        instances_to_delete: InstancesPerDerivationPresetPerFactorSource,
    ) -> InstancesInCacheConsumer {
        let instances_clone = instances_to_delete.clone();
        let cache_client_clone = self.cache_client.clone();
        InstancesInCacheConsumer::new(move || {
            let cache_client_clone_clone = cache_client_clone.clone();
            let instances_clone_clone = instances_clone.clone();
            async move {
                cache_client_clone_clone.delete(instances_clone_clone).await
            }
        })
    }

    async fn _provide_for_presets(
        &mut self,
        quantified_derivation_presets: IdentifiedVecOf<
            QuantifiedDerivationPreset,
        >,
        derivation_purpose: DerivationPurpose,
    ) -> Result<(
        InstancesInCacheConsumer,
        InternalFactorInstancesProviderOutcome,
    )> {
        println!(
            "🌮 FIP - _provide_for_presets: {:?}, purpose: {:?}",
            quantified_derivation_presets, derivation_purpose
        );

        let factor_sources = self.factor_sources.clone();
        let network_id = self.network_id;

        println!("🌮 FIP - _provide_for_presets - calling `cache_client.get`");
        let cached = self
            .cache_client
            .get(
                &factor_sources.iter().map(|f| f.id_from_hash()).collect(),
                quantified_derivation_presets.clone(),
                network_id,
            )
            .await?;

        match cached {
            CachedInstancesWithQuantitiesOutcome::Satisfied(
                enough_instances,
            ) => {
                println!("🌮 FIP - _provide_for_presets - cached outcome - ✅ SATISFIED ✅");
                // When/if caller calls `instances_in_cache_consumer.consume()` the `enough_instances`
                // will be deleted from the cache, they are still present in the cache now
                // and will continue to be present until the `consume()` is called.
                let instances_in_cache_consumer = self
                    .make_instances_in_cache_consumer(
                        enough_instances.clone().cached,
                    );
                Ok((
                    instances_in_cache_consumer,
                    InternalFactorInstancesProviderOutcome::satisfied_by_cache(
                        enough_instances,
                    ),
                ))
            }
            CachedInstancesWithQuantitiesOutcome::NotSatisfied(unsatisfied) => {
                println!("🌮 FIP - _provide_for_presets - cached outcome - 🙅🏻‍♀️ NotSatisfied => `derive_more_and_cache` 🙅🏻‍♀️ ");
                self.derive_more_and_cache(
                    &quantified_derivation_presets,
                    unsatisfied,
                    derivation_purpose,
                )
                .await
            }
        }
    }

    async fn derive_more_and_cache(
        &mut self,
        requested_quantified_derivation_presets: &IdentifiedVecOf<
            QuantifiedDerivationPreset,
        >,
        not_satisfied: CacheNotSatisfied,
        derivation_purpose: DerivationPurpose,
    ) -> Result<(
        InstancesInCacheConsumer,
        InternalFactorInstancesProviderOutcome,
    )> {

        let remaining_quantities_to_derive =
            not_satisfied.remaining_quantities_to_derive();

            assert!(!remaining_quantities_to_derive.is_empty(), "❌ No instances to derive?");
       
        println!("🌮 FIP - derive_more_and_cache - deriving more, specifically: {:?}", remaining_quantities_to_derive);

        let pdp_pf_newly_derived = self
            .derive_more(remaining_quantities_to_derive, derivation_purpose)
            .await?;

        assert!(!pdp_pf_newly_derived.is_empty(), "❌ Failed to derive instances?");

        for (k, v) in pdp_pf_newly_derived.iter() {
            for (x, y) in v.iter() {
                println!("🌮 FIP 🔮✨ derived - (preset: {:?}, factor: {:?}) - #{:?} instances ✨🔮", k, x, y.len());
            }
        }

        let pdp_pf_found_in_cache_leq_requested =
            not_satisfied.cached_instances_to_use();

        let Split {
            pdp_pf_to_use_directly,
            pdp_pf_to_cache,
        } = self.split(
            requested_quantified_derivation_presets,
            &pdp_pf_found_in_cache_leq_requested,
            &pdp_pf_newly_derived,
        );

        let instances_in_cache_consumer = self
            .make_instances_in_cache_consumer(
                pdp_pf_found_in_cache_leq_requested.clone(),
            );

        self.cache_client.insert(&pdp_pf_to_cache).await?;

        let outcome = InternalFactorInstancesProviderOutcome::transpose(
            pdp_pf_to_cache,
            pdp_pf_to_use_directly,
            pdp_pf_found_in_cache_leq_requested,
            pdp_pf_newly_derived,
        );

        Ok((instances_in_cache_consumer, outcome))
    }

    /// Per factor, split the instances into those to use directly and those to cache.
    /// based on the originally requested quantity.
    fn split(
        &self,
        requested_quantified_derivation_presets: &IdentifiedVecOf<
            QuantifiedDerivationPreset,
        >,
        pdp_pf_found_in_cache_leq_requested: &InstancesPerDerivationPresetPerFactorSource,
        pdp_pf_newly_derived: &InstancesPerDerivationPresetPerFactorSource,
    ) -> Split {

        let derivation_presets_of_instances_to_merge = pdp_pf_found_in_cache_leq_requested
            .keys()
            .chain(pdp_pf_newly_derived.keys())
            .cloned()
            .collect::<IndexSet<DerivationPreset>>();

        // Start by merging the instances found in cache and the newly derived instances,
        // into a single collection of instances per factor source, with the
        // instances from cache first in the list (per factor), and then the newly derived.
        // this is important so that we consume the instances from cache first.
        let pdp_pf_derived_appended_to_from_cache = derivation_presets_of_instances_to_merge
            .iter()
            .filter_map(|preset| {
                let pf_found_in_cache_leq_requested =
                    pdp_pf_found_in_cache_leq_requested
                        .get(preset)
                        .cloned()
                        // can be nil -> empty, if no instance was found in cache for this preset!
                        .unwrap_or_default();
                    
                let pf_newly_derived = pdp_pf_newly_derived
                    .get(preset)
                    .cloned()
                    // can be nil -> empty, if we did not derive any new instance for this preset!
                    .unwrap_or_default();

                if pf_found_in_cache_leq_requested.is_empty()
                    && pf_newly_derived.is_empty()
                {
                    panic!("is this possible? apparently! remove this panic and rely on the filter_map...");
                    return None;
                }

                let pf_instances = self
                    .factor_sources
                    .clone()
                    .into_iter()
                    .map(|f| f.id_from_hash())
                    .map(|factor_source_id| {
                        
                        let mut merged = IndexSet::new();
                       
                       let from_cache = pf_found_in_cache_leq_requested
                            .get(&factor_source_id)
                            .cloned()
                            .unwrap_or_default();
                        
                        let newly_derived = pf_newly_derived
                            .get(&factor_source_id)
                            .cloned()
                            .unwrap_or_default();

                        println!("🌮 FIP - split (preset: {:?}, factor: {:?}) - `from_cache`: #{:?}", preset, factor_source_id, from_cache.len());
                        println!("🌮 FIP - split (preset: {:?}, factor: {:?}) - `newly_derived`: #{:?}", preset, factor_source_id, newly_derived.len());
                        println!("🌮 FIP - split (preset: {:?}, factor: {:?}) - `merged`: #{:?}", preset, factor_source_id, merged.len());
                        
                        // IMPORTANT: Must put instances from cache **first**...
                        merged.extend(from_cache);
                        
                        // ... and THEN the newly derived, so we consume the ones with
                        // lower index from cache first.
                        merged.extend(newly_derived);

                        assert!(
                            merged
                            .clone()
                            .into_iter()
                            .all(|f| 
                                DerivationPreset::try_from(f.derivation_path().agnostic()).unwrap() == *preset
                            )
                        );
                        assert!(!merged.is_empty());
                            (factor_source_id, FactorInstances::from(merged))

                    })
                    .collect::<IndexMap<FactorSourceIDFromHash, FactorInstances>>();

           assert!(!pf_instances.is_empty());
                        Some((*preset, pf_instances))

            })
            .collect::<InstancesPerDerivationPresetPerFactorSource>();

        let mut pdp_pf_to_use_directly =
            InstancesPerDerivationPresetPerFactorSource::new();
        let mut pdp_pf_to_cache =
            InstancesPerDerivationPresetPerFactorSource::new();

   
        let originally_requested_presets = requested_quantified_derivation_presets
            .iter()
            .map(|qdp| qdp.derivation_preset)
            .collect::<IndexSet<DerivationPreset>>();

        // Using the merged map, split the instances into those to use directly and those to cache.
        for (preset, pf_derived_appended_to_from_cache) in
            pdp_pf_derived_appended_to_from_cache
        {
            let mut pf_to_cache =
            IndexMap::<FactorSourceIDFromHash, FactorInstances>::new();

            let mut pf_to_use_directly = IndexMap::new();


            for (factor, instances) in
                pf_derived_appended_to_from_cache.clone().into_iter()
            {
                assert!(
                    instances.factor_instances()
                    .into_iter()
                    .all(|f| 
                        DerivationPreset::try_from(f.derivation_path().agnostic()).unwrap() == preset
                    )
                );

                assert!(
                    instances.factor_instances()
                    .into_iter()
                    .all(|f| 
                      f.factor_source_id() == factor
                    )
                );

                println!(
                    "🌮 FIP - split (preset: {:?}, factor: {:?}) - instances: #{:?}",
                    preset, factor,
                    instances.len()
                );
           

                if originally_requested_presets.contains(&preset) {
                    // might have to split
           

                 
    
                    let requested_quantified_derivation_preset =
                        requested_quantified_derivation_presets.get_id(&preset).unwrap();

                        println!("🌮 FIP - split - requested_quantified_derivation_preset: {:?}", requested_quantified_derivation_preset);
    
                        let instances_relevant_to_use_directly_with_abundance = instances;
    
                        println!("🌮 FIP - split - #instances_relevant_to_use_directly_with_abundance: {:?}", instances_relevant_to_use_directly_with_abundance.len());
    
                        let originally_requested_quantity =
                            requested_quantified_derivation_preset.quantity;
    
                        println!(
                            "🌮 FIP - split - originally_requested_quantity: {:?}",
                            originally_requested_quantity
                        );

                        if originally_requested_quantity > instances_relevant_to_use_directly_with_abundance.len() {
                            println!("🌮 FIP - split ❌❌ not enough instances to use directly! (preset: {:?}, factor: {:?}) ❌❌", preset, factor);
                        }
    
                        let (to_use_directly, to_cache) =
                            instances_relevant_to_use_directly_with_abundance
                                .split_at(originally_requested_quantity);
    
                        println!(
                            "🌮 FIP - split - to_use_directly: #{:?}",
                            to_use_directly.len()
                        );
    
                        println!(
                            "🌮 FIP - split - to_cache: #{:?}",
                            to_cache.len()
                        );
    
                        pf_to_use_directly
                            .insert(factor, to_use_directly);

                        pf_to_cache.insert(factor, to_cache);


                } else {
                    // easy case, we don't want to use this directly at all
                    // meaning all
                    pf_to_cache.insert(factor, instances);
                    // we do not add any FactorInstances to `pf_to_use_directly` for this factor
                }
                
            }

            pdp_pf_to_use_directly.insert(preset, pf_to_use_directly);
            pdp_pf_to_cache.insert(preset, pf_to_cache);
        }

        Split {
            pdp_pf_to_use_directly,
            pdp_pf_to_cache,
        }
    }

    pub(super) async fn derive_more(
        &self,
        quantities_to_derive: QuantitiesToDerive,
        derivation_purpose: DerivationPurpose,
    ) -> Result<InstancesPerDerivationPresetPerFactorSource> {
        let factor_sources = self.factor_sources.clone();
        let network_id = self.network_id;

        let cache_snapshot = self.cache_client.snapshot().await?;
        let next_index_assigner = NextDerivationEntityIndexAssigner::new(
            network_id,
            self.profile.clone(),
            cache_snapshot,
        );

        let per_preset_per_factor_paths = quantities_to_derive
            .into_iter()
            .filter(|(_, per_factor_source)| !per_factor_source.is_empty())
            .map(|(derivation_preset, per_factor_source)| {
                let per_factor_paths = per_factor_source
                    .into_iter()
                    .map(|(factor_source_id, qty)| {
                        assert!(qty > 0);
                        // `qty` many paths
                        let paths = (0..qty)
                            .map(|_| {
                                let index_agnostic_path = derivation_preset
                                    .index_agnostic_path_on_network(network_id);
                                let path = next_index_assigner
                                    .next(factor_source_id, index_agnostic_path)
                                    .map(|index| {
                                        DerivationPath::from((
                                            index_agnostic_path,
                                            index,
                                        ))
                                    })?;
                                Ok(path)
                            })
                            .collect::<Result<IndexSet<DerivationPath>>>()?;

                            assert!(!paths.is_empty());

                        Ok((factor_source_id, paths))
                    })
                    .collect::<Result<
                        IndexMap<
                            FactorSourceIDFromHash,
                            IndexSet<DerivationPath>,
                        >,
                    >>()?;

                Ok((derivation_preset, per_factor_paths))
            })
            .collect::<Result<
                // IndexMap<FactorSourceIDFromHash, IndexSet<DerivationPath>>,
                IndexMap<
                    DerivationPreset,
                    IndexMap<FactorSourceIDFromHash, IndexSet<DerivationPath>>,
                >,
            >>()?;

        let mut per_factor_paths =
            IndexMap::<FactorSourceIDFromHash, IndexSet<DerivationPath>>::new();

        for (_, pf) in per_preset_per_factor_paths.clone() {
            for (factor_source_id, paths) in pf {
                per_factor_paths.append_or_insert_to(factor_source_id, paths);
            }
        }

        for (k, v) in per_factor_paths.iter() {
            let contains_rola_path = v.iter().any(|p| p.agnostic().key_kind == CAP26KeyKind::AuthenticationSigning);
            println!("🌮 FIP - derive_more - 🛣️ per_factor_paths - (factor: {:?}) - v.len(): #{:?} contains_rola_path? {:?}", k, v.len(), contains_rola_path);
        }
        println!("🛡️ #factor_sources: {:?}", factor_sources.len());

        // println!(
        //     "🌮 FIP - derive_more - per_factor_paths: #{:?}",
        //     per_factor_paths
        //         .clone()
        //         .values()
        //         .fold(0, |acc, e| acc + e.len())
        // );

        let interactor = self.interactor.clone();

        let collector = KeysCollector::new(
            factor_sources,
            per_factor_paths,
            interactor,
            derivation_purpose,
        )?;

        let pf_derived = collector.collect_keys().await.factors_by_source;

        for (k, v) in pf_derived.iter() {
            println!("🌮 FIP - derive_more - 🔮🦄🍬 derived - (factor: {:?}) - v.len(): #{:?}", k, v.len());
        }

        assert!(!pf_derived.is_empty(), "❌❌ no instances derived ❌");

        let pf_pdp_derived = pf_derived
            .into_iter()
            .filter_map(|(k, v)| {
                if v.is_empty() {
                    println!("🌮 FIP - derive_more EMPTY 🐌 for factor: {:?})", k);
                    return None;
                }
                println!("🌮 FIP - derive_more - derived 🛡️ - (factor: {:?}) - v.len(): #{:?}", k, v.len());
             let apor =      InstancesByDerivationPreset::from(FactorInstances::from(v))
             .0;
            println!("🌮 FIP - derive_more - derived 🦧 - (factor: {:?}) - apor.len() #{:?}", k, apor.len());
            if apor.is_empty() {
                None
            } else {
                Some((
                    k,
               apor
                ))
            }
            })
            .collect::<IndexMap<
                FactorSourceIDFromHash,
                IndexMap<DerivationPreset, FactorInstances>,
            >>();

        // we need to transpose the `pf_pdp_derived`
        assert!(!pf_pdp_derived.is_empty(), "🙅🏻‍♀️ no instances derived?");

        let mut pdp_pf_instances = IndexMap::<
            DerivationPreset,
            IndexMap<FactorSourceIDFromHash, FactorInstances>,
        >::new();

        for (factor_source_id, pdp) in pf_pdp_derived {
            assert!(!pdp.is_empty(), "🙅🏻‍♀️ no instances for factor: {factor_source_id}");
            for (preset, instances) in pdp {
                assert!(!instances.is_empty(), "🙅🏻‍♀️ no instances for preset: {preset:?}");
                pdp_pf_instances.append_or_insert_to(
                    preset,
                    IndexMap::<FactorSourceIDFromHash, FactorInstances>::kv(
                        factor_source_id,
                        instances,
                    ),
                );
            }
        }

        // for (preset, per_factor) in per_preset_per_factor_paths {
        //     let mut pf_instances =
        //         IndexMap::<FactorSourceIDFromHash, FactorInstances>::new();

        //     for (factor_source_id, paths) in per_factor {
        //         let derived_for_factor = pf_derived
        //             .get(&factor_source_id)
        //             .cloned()
        //             .unwrap_or_default(); // if None -> Empty -> fail below.

        //         if derived_for_factor.len() < paths.len() {
        //             return Err(CommonError::FactorInstancesProviderDidNotDeriveEnoughFactors);
        //         }

        //         pf_instances.insert(
        //             factor_source_id,
        //             derived_for_factor.into_iter().collect::<FactorInstances>(),
        //         );
        //     }

        //     pdp_pf_instances.insert(preset, pf_instances);
        // }

        Ok(pdp_pf_instances)
    }
}

/// A split of FactorInstances per DerivationPreset per FactorSource
/// is splitting and newly derived FactorInstances and then pre-pending
/// any instances found in cache to `pdp_pf_to_use_directly`.
struct Split {
    /// Per DerivationPreset per FactorSource instances to use directly
    pdp_pf_to_use_directly: InstancesPerDerivationPresetPerFactorSource,
    /// Per DerivationPreset per FactorSource instances to cache
    pdp_pf_to_cache: InstancesPerDerivationPresetPerFactorSource,
}
