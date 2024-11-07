use std::sync::{Arc, RwLock};

use crate::prelude::*;
use crate::{factor_instances_provider::next_index_assigner, prelude::*};
use itertools::cloned;

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
pub struct FactorInstancesProvider<'a, 'b> {
    network_id: NetworkID,
    factor_sources: IndexSet<FactorSource>,
    profile: Option<&'b Profile>,
    cache_client: &'a FactorInstancesCacheClient,
    interactors: Arc<dyn KeysDerivationInteractors>,
}

/// ===============
/// PUBLIC
/// ===============
impl<'a, 'b> FactorInstancesProvider<'a, 'b> {
    pub fn new(
        network_id: NetworkID,
        factor_sources: IndexSet<FactorSource>,
        profile: impl Into<Option<&'b Profile>>,
        cache_client: &'a FactorInstancesCacheClient,
        interactors: Arc<dyn KeysDerivationInteractors>,
    ) -> Self {
        Self {
            network_id,
            factor_sources,
            profile: profile.into(),
            cache_client,
            interactors,
        }
    }

    pub async fn provide(
        self,
        quantified_derivation_preset: QuantifiedDerivationPreset,
    ) -> Result<InternalFactorInstancesProviderOutcome> {
        let mut _self = self;

        _self._provide(quantified_derivation_preset).await
    }
}

/// Uses a `FactorInstancesProvider` to fill the cache with instances for a new FactorSource.
pub struct CacheFiller;
impl CacheFiller {
    /// Uses a `FactorInstancesProvider` to fill the `cache` with FactorInstances for a new FactorSource.
    /// Saves FactorInstances into the mutable `cache` parameter and returns a
    /// copy of the instances.
    pub async fn for_new_factor_source(
        cache_client: &FactorInstancesCacheClient,
        profile: Option<&Profile>,
        factor_source: FactorSource,
        network_id: NetworkID, // typically mainnet
        interactors: Arc<dyn KeysDerivationInteractors>,
    ) -> Result<FactorInstancesProviderOutcomeForFactor> {
        let provider = FactorInstancesProvider::new(
            network_id,
            IndexSet::just(factor_source.clone()),
            profile,
            cache_client,
            interactors,
        );
        let quantities = IndexMap::kv(
            factor_source.id_from_hash(),
            DerivationPreset::all()
                .into_iter()
                .map(|dp| (dp, CACHE_FILLING_QUANTITY))
                .collect::<IndexMap<DerivationPreset, usize>>(),
        );
        let derived = provider.derive_more(quantities).await?;

        cache_client.insert_all(&derived).await?;

        let derived =
            derived.get(&factor_source.id_from_hash()).unwrap().clone();
        let outcome = InternalFactorInstancesProviderOutcomeForFactor::new(
            factor_source.id_from_hash(),
            derived.clone(),
            FactorInstances::default(),
            FactorInstances::default(),
            derived,
        );
        Ok(outcome.into())
    }
}

/// ===============
/// Private
/// ===============
impl<'a, 'b> FactorInstancesProvider<'a, 'b> {
    async fn _provide(
        &mut self,
        quantified_derivation_preset: QuantifiedDerivationPreset,
    ) -> Result<InternalFactorInstancesProviderOutcome> {
        let factor_sources = self.factor_sources.clone();
        let network_id = self.network_id;
        println!("🐉 provider: reading from cache");
        let cached = self
            .cache_client
            .get_poly_factor_with_quantities(
                &factor_sources.iter().map(|f| f.id_from_hash()).collect(),
                &quantified_derivation_preset,
                network_id,
            )
            .await?;
        println!("🐉 provider: read from cache: {:?}", cached);

        match cached {
            CachedInstancesWithQuantitiesOutcome::Satisfied(
                enough_instances,
            ) => {
                // Remove the instances which are going to be used from the cache
                // since we only peeked at them.
                self.cache_client.delete(&enough_instances).await?;
                Ok(InternalFactorInstancesProviderOutcome::satisfied_by_cache(
                    enough_instances,
                ))
            }
            CachedInstancesWithQuantitiesOutcome::NotSatisfied {
                quantities_to_derive,
                partial_instances,
            } => {
                self.derive_more_and_cache(
                    quantified_derivation_preset,
                    partial_instances,
                    quantities_to_derive,
                )
                .await
            }
        }
    }

    async fn derive_more_and_cache(
        &mut self,
        quantified_derivation_preset: QuantifiedDerivationPreset,
        pf_found_in_cache_leq_requested: IndexMap<
            FactorSourceIDFromHash,
            FactorInstances,
        >,
        pf_pdp_qty_to_derive: IndexMap<
            FactorSourceIDFromHash,
            IndexMap<DerivationPreset, usize>,
        >,
    ) -> Result<InternalFactorInstancesProviderOutcome> {
        println!("🐉 provider - 🔮 derive more: START\nquantified_derivation_preset: {:?}\npf_pdp_qty_to_derive: {:?}", quantified_derivation_preset, pf_pdp_qty_to_derive);

        let pf_newly_derived = self.derive_more(pf_pdp_qty_to_derive).await?;

        println!(
            "🐉 provider - 🔮 derived more: #{:?}",
            pf_newly_derived.len()
        );

        let Split {
            pf_to_use_directly,
            pf_to_cache,
        } = self.split(
            &quantified_derivation_preset,
            &pf_found_in_cache_leq_requested,
            &pf_newly_derived,
        );

        println!(
            "🐉 provider - 🔮 derive more, SPLIT: {:?}",
            pf_to_use_directly
        );

        self.cache_client
            .delete(&pf_found_in_cache_leq_requested)
            .await?;
        self.cache_client.insert_all(&pf_to_cache).await?;

        let outcome = InternalFactorInstancesProviderOutcome::transpose(
            pf_to_cache,
            pf_to_use_directly,
            pf_found_in_cache_leq_requested,
            pf_newly_derived,
        );
        let outcome = outcome;
        Ok(outcome)
    }

    /// Per factor, split the instances into those to use directly and those to cache.
    /// based on the originally requested quantity.
    fn split(
        &self,
        originally_requested_quantified_derivation_preset: &QuantifiedDerivationPreset,
        pf_found_in_cache_leq_requested: &IndexMap<
            FactorSourceIDFromHash,
            FactorInstances,
        >,
        pf_newly_derived: &IndexMap<FactorSourceIDFromHash, FactorInstances>,
    ) -> Split {
        // Start by merging the instances found in cache and the newly derived instances,
        // into a single collection of instances per factor source, with the
        // instances from cache first in the list (per factor), and then the newly derived.
        // this is important so that we consume the instances from cache first.
        let pf_derived_appended_to_from_cache = self
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
                // IMPORTANT: Must put instances from cache **first**...
                merged.extend(from_cache);
                // ... and THEN the newly derived, so we consume the ones with
                // lower index from cache first.
                merged.extend(newly_derived);

                (factor_source_id, FactorInstances::from(merged))
            })
            .collect::<IndexMap<FactorSourceIDFromHash, FactorInstances>>();

        let mut pf_to_use_directly = IndexMap::new();
        let mut pf_to_cache =
            IndexMap::<FactorSourceIDFromHash, FactorInstances>::new();
        let quantity_originally_requested =
            originally_requested_quantified_derivation_preset.quantity;
        let preset_originally_requested =
            originally_requested_quantified_derivation_preset.derivation_preset;

        // Using the merged map, split the instances into those to use directly and those to cache.
        for (factor_source_id, instances) in
            pf_derived_appended_to_from_cache.clone().into_iter()
        {
            let mut instances_by_derivation_preset =
                InstancesByDerivationPreset::from(instances);

            if let Some(instances_relevant_to_use_directly_with_abundance) =
                instances_by_derivation_preset
                    .remove(preset_originally_requested)
            {
                let (to_use_directly, to_cache) =
                    instances_relevant_to_use_directly_with_abundance
                        .split_at(quantity_originally_requested);
                pf_to_use_directly.insert(factor_source_id, to_use_directly);
                pf_to_cache.insert(factor_source_id, to_cache);
            }

            pf_to_cache.append_or_insert_to(
                factor_source_id,
                instances_by_derivation_preset.all_instances(),
            );
        }

        Split {
            pf_to_use_directly,
            pf_to_cache,
        }
    }

    async fn derive_more(
        &self,
        pf_pdp_quantity_to_derive: IndexMap<
            FactorSourceIDFromHash,
            IndexMap<DerivationPreset, usize>,
        >,
    ) -> Result<IndexMap<FactorSourceIDFromHash, FactorInstances>> {
        println!(
            "🤡 derive more: pf_pdp_quantity_to_derive {:?}",
            pf_pdp_quantity_to_derive
        );
        let factor_sources = self.factor_sources.clone();
        let network_id = self.network_id;

        println!("🤡 derive more: creating next index assigner",);
        let cache_snapshot = self.cache_client.snapshot().await?;
        let next_index_assigner = NextDerivationEntityIndexAssigner::new(
            network_id,
            self.profile,
            cache_snapshot,
        );

        println!("🤡 derive more: created next index assigner",);

        let pf_paths = pf_pdp_quantity_to_derive
            .into_iter()
            .map(|(factor_source_id, pdp_quantity_to_derive)| {
                let paths = pdp_quantity_to_derive
                    .into_iter()
                    .map(|(derivation_preset, qty)| {
                        // `qty` many paths
                        let paths = (0..qty)
                            .map(|_| {
                                // println!("🤡 derive more: creating next...offset: {:?}", offset);
                                let index_agnostic_path = derivation_preset
                                    .index_agnostic_path_on_network(network_id);
                                // println!("🤡 derive more: index_agnostic_path: {:?}", index_agnostic_path);
                                let index = next_index_assigner.next(
                                    factor_source_id,
                                    index_agnostic_path,
                                )?;
                                // println!("🤡 derive more: index from next_index_assigner {:?}", index);
                                let derivation_path = DerivationPath::from((
                                    index_agnostic_path,
                                    index,
                                ));
                                // println!("🤡 derive more: path {:?}", derivation_path);
                                Ok(derivation_path)
                            })
                            .collect::<Result<IndexSet<DerivationPath>>>()?;

                        Ok(paths)
                    })
                    .collect::<Result<Vec<IndexSet<DerivationPath>>>>()?;

                // flatten (I was unable to use `flat_map` above combined with `Result`...)
                let paths =
                    paths.into_iter().flatten().collect::<IndexSet<_>>();

                Ok((factor_source_id, paths))
            })
            .collect::<Result<
                IndexMap<FactorSourceIDFromHash, IndexSet<DerivationPath>>,
            >>()?;

        println!("🤡 derive more: paths #{:?}", pf_paths.len());

        let keys_collector = KeysCollector::new(
            factor_sources,
            pf_paths.clone(),
            self.interactors.clone(),
        )?;

        let pf_derived = keys_collector.collect_keys().await.factors_by_source;

        let mut pf_instances =
            IndexMap::<FactorSourceIDFromHash, FactorInstances>::new();

        for (factor_source_id, paths) in pf_paths {
            let derived_for_factor = pf_derived
                .get(&factor_source_id)
                .cloned()
                .unwrap_or_default(); // if None -> Empty -> fail below.
            if derived_for_factor.len() < paths.len() {
                return Err(CommonError::FactorInstancesProviderDidNotDeriveEnoughFactors);
            }
            pf_instances.insert(
                factor_source_id,
                derived_for_factor.into_iter().collect::<FactorInstances>(),
            );
        }

        Ok(pf_instances)
    }
}

struct Split {
    pf_to_use_directly: IndexMap<FactorSourceIDFromHash, FactorInstances>,
    pf_to_cache: IndexMap<FactorSourceIDFromHash, FactorInstances>,
}
