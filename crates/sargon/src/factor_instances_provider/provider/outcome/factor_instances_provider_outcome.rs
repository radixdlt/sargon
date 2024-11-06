use crate::prelude::*;

/// Identical to `InternalFactorInstancesProviderOutcome` but `FactorInstancesProviderOutcomeForFactor` instead of `InternalFactorInstancesProviderOutcomeForFactor`, having
/// renamed field values to make it clear that `to_cache` instances  already have been cached.
#[derive(Clone, Debug)]
pub struct FactorInstancesProviderOutcome {
    pub per_factor: IndexMap<
        FactorSourceIDFromHash,
        FactorInstancesProviderOutcomeForFactor,
    >,
}

impl From<InternalFactorInstancesProviderOutcome>
    for FactorInstancesProviderOutcome
{
    fn from(value: InternalFactorInstancesProviderOutcome) -> Self {
        Self {
            per_factor: value
                .per_factor
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

#[cfg(test)]
impl FactorInstancesProviderOutcome {
    pub fn newly_derived_instances_from_all_factor_sources(
        &self,
    ) -> FactorInstances {
        self.per_factor
            .values()
            .flat_map(|x| x.debug_was_derived.factor_instances())
            .collect()
    }

    pub fn total_number_of_newly_derived_instances(&self) -> usize {
        self.newly_derived_instances_from_all_factor_sources().len()
    }

    pub fn derived_any_new_instance_for_any_factor_source(&self) -> bool {
        self.total_number_of_newly_derived_instances() > 0
    }

    pub fn instances_found_in_cache_from_all_factor_sources(
        &self,
    ) -> FactorInstances {
        self.per_factor
            .values()
            .flat_map(|x| x.debug_found_in_cache.factor_instances())
            .collect()
    }

    pub fn total_number_of_instances_found_in_cache(&self) -> usize {
        self.instances_found_in_cache_from_all_factor_sources()
            .len()
    }

    pub fn found_any_instances_in_cache_for_any_factor_source(&self) -> bool {
        self.total_number_of_instances_found_in_cache() > 0
    }
}
