use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheInstancesAndRemainingQuantityToDerive {
    pub instances_to_use_from_cache: FactorInstances, // if empty then this was not a requested derivation preset, but we are cache filling and found `quantity_to_derive` needed to fill cache.
    pub quantity_to_derive: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheNotSatisfied {
    /// PER DerivationPreset => PER FactorSourceID => CacheInstancesAndRemainingQuantityToDerive
    pub cached_and_quantities_to_derive: IndexMap<
        DerivationPreset,
        IndexMap<
            FactorSourceIDFromHash,
            CacheInstancesAndRemainingQuantityToDerive,
        >,
    >,
}
impl CacheNotSatisfied {
    fn map<R>(
        &self,
        extract: impl Fn(
            (
                FactorSourceIDFromHash,
                CacheInstancesAndRemainingQuantityToDerive,
            ),
        ) -> Option<(FactorSourceIDFromHash, R)>,
    ) -> IndexMap<DerivationPreset, IndexMap<FactorSourceIDFromHash, R>> {
        self.cached_and_quantities_to_derive
            .clone()
            .into_iter()
            .filter_map(|(preset, v)| {
                let per_factor = v
                    .into_iter()
                    .filter_map(|(x, y)| extract((x, y)))
                    .collect::<IndexMap<FactorSourceIDFromHash, R>>();

                if per_factor.is_empty() {
                    None
                } else {
                    Some((preset, per_factor))
                }
            })
            .collect()
    }

    pub fn cached_instances_to_use(
        &self,
    ) -> InstancesPerDerivationPresetPerFactorSource {
        self.map(|(x, y)| {
            let instances = y.instances_to_use_from_cache;
            if instances.is_empty() {
                None
            } else {
                Some((x, instances))
            }
        })
    }

    pub fn remaining_quantities_to_derive(&self) -> QuantitiesToDerive {
        self.map(|(x, y)| {
            if y.quantity_to_derive > 0 {
                Some((x, y.quantity_to_derive))
            } else {
                None
            }
        })
    }
}
pub type QuantitiesToDerive =
    IndexMap<DerivationPreset, IndexMap<FactorSourceIDFromHash, usize>>;

pub type InstancesPerDerivationPresetPerFactorSource = IndexMap<
    DerivationPreset,
    IndexMap<FactorSourceIDFromHash, FactorInstances>,
>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheSatisfied {
    /// PER DerivationPreset => PER FactorSourceID => FactorInstances
    pub cached: IndexMap<
        DerivationPreset,
        IndexMap<FactorSourceIDFromHash, FactorInstances>,
    >,
}

#[derive(Debug, Clone, PartialEq, Eq, enum_as_inner::EnumAsInner)]
pub enum CachedInstancesWithQuantitiesOutcome {
    Satisfied(CacheSatisfied),
    NotSatisfied(CacheNotSatisfied),
}
