use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct InternalFactorInstancesProviderOutcome {
    pub per_factor:
        IndexMap<FactorSourceIDFromHash, InternalFactorInstancesProviderOutcomeForFactor>,
}

impl InternalFactorInstancesProviderOutcome {
    pub fn new(
        per_factor: IndexMap<
            FactorSourceIDFromHash,
            InternalFactorInstancesProviderOutcomeForFactor,
        >,
    ) -> Self {
        Self { per_factor }
    }

    /// Outcome of FactorInstances just from cache, none have been derived.
    pub fn satisfied_by_cache(
        pf_found_in_cache: IndexMap<FactorSourceIDFromHash, FactorInstances>,
    ) -> Self {
        Self::new(
            pf_found_in_cache
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        InternalFactorInstancesProviderOutcomeForFactor::satisfied_by_cache(k, v),
                    )
                })
                .collect(),
        )
    }

    /// "Transposes" a **collection** of `IndexMap<FactorSourceID, FactorInstances>` into `IndexMap<FactorSourceID, **collection** FactorInstances>` (`InternalFactorInstancesProviderOutcomeForFactor` is essentially a collection of FactorInstance)
    pub fn transpose(
        pf_to_cache: IndexMap<FactorSourceIDFromHash, FactorInstances>,
        pf_to_use_directly: IndexMap<FactorSourceIDFromHash, FactorInstances>,
        pf_found_in_cache: IndexMap<FactorSourceIDFromHash, FactorInstances>,
        pf_newly_derived: IndexMap<FactorSourceIDFromHash, FactorInstances>,
    ) -> Self {
        struct Builder {
            factor_source_id: FactorSourceIDFromHash,

            /// Might be empty
            pub to_cache: IndexSet<HierarchicalDeterministicFactorInstance>,
            /// Might be empty
            pub to_use_directly: IndexSet<HierarchicalDeterministicFactorInstance>,

            /// LESS IMPORTANT - for tests...
            /// might overlap with `to_use_directly`
            pub found_in_cache: IndexSet<HierarchicalDeterministicFactorInstance>,
            /// might overlap with `to_cache` and `to_use_directly`
            pub newly_derived: IndexSet<HierarchicalDeterministicFactorInstance>,
        }
        impl Builder {
            fn build(self) -> InternalFactorInstancesProviderOutcomeForFactor {
                let to_cache = FactorInstances::from(self.to_cache);
                let to_use_directly = FactorInstances::from(self.to_use_directly);
                let found_in_cache = FactorInstances::from(self.found_in_cache);
                let newly_derived = FactorInstances::from(self.newly_derived);
                InternalFactorInstancesProviderOutcomeForFactor::new(
                    self.factor_source_id,
                    to_cache,
                    to_use_directly,
                    found_in_cache,
                    newly_derived,
                )
            }
            fn new(factor_source_id: FactorSourceIDFromHash) -> Self {
                Self {
                    factor_source_id,
                    to_cache: IndexSet::new(),
                    to_use_directly: IndexSet::new(),
                    found_in_cache: IndexSet::new(),
                    newly_derived: IndexSet::new(),
                }
            }
        }
        let mut builders = IndexMap::<FactorSourceIDFromHash, Builder>::new();

        for (factor_source_id, instances) in pf_found_in_cache {
            if let Some(builder) = builders.get_mut(&factor_source_id) {
                builder.found_in_cache.extend(instances.factor_instances());
            } else {
                let mut builder = Builder::new(factor_source_id);
                builder.found_in_cache.extend(instances.factor_instances());
                builders.insert(factor_source_id, builder);
            }
        }

        for (factor_source_id, instances) in pf_newly_derived {
            if let Some(builder) = builders.get_mut(&factor_source_id) {
                builder.newly_derived.extend(instances.factor_instances());
            } else {
                let mut builder = Builder::new(factor_source_id);
                builder.newly_derived.extend(instances.factor_instances());
                builders.insert(factor_source_id, builder);
            }
        }

        for (factor_source_id, instances) in pf_to_cache {
            if let Some(builder) = builders.get_mut(&factor_source_id) {
                builder.to_cache.extend(instances.factor_instances());
            } else {
                let mut builder = Builder::new(factor_source_id);
                builder.to_cache.extend(instances.factor_instances());
                builders.insert(factor_source_id, builder);
            }
        }

        for (factor_source_id, instances) in pf_to_use_directly {
            if let Some(builder) = builders.get_mut(&factor_source_id) {
                builder.to_use_directly.extend(instances.factor_instances());
            } else {
                let mut builder = Builder::new(factor_source_id);
                builder.to_use_directly.extend(instances.factor_instances());
                builders.insert(factor_source_id, builder);
            }
        }

        Self::new(
            builders
                .into_iter()
                .map(|(k, v)| (k, v.build()))
                .collect::<IndexMap<FactorSourceIDFromHash, InternalFactorInstancesProviderOutcomeForFactor>>(),
        )
    }
}
/*
#[cfg(test)]
mod tests {
    use super::*;
    type Sut = InternalFactorInstancesProviderOutcome;

    #[test]
    fn only_to_cache() {
        let i = HierarchicalDeterministicFactorInstance::fia0();

        let sut = Sut::transpose(
            IndexMap::kv(
                FactorSourceIDFromHash::sample_at(0),
                FactorInstances::just(i.clone()),
            ),
            IndexMap::new(),
            IndexMap::new(),
            IndexMap::new(),
        );
        assert_eq!(
            sut.per_factor.get(&i.factor_source_id()).unwrap().to_cache,
            FactorInstances::just(i)
        )
    }

    #[test]
    fn only_to_use_directly() {
        let i = HierarchicalDeterministicFactorInstance::fia0();

        let sut = Sut::transpose(
            IndexMap::new(),
            IndexMap::kv(
                FactorSourceIDFromHash::sample_at(0),
                FactorInstances::just(i.clone()),
            ),
            IndexMap::new(),
            IndexMap::new(),
        );
        assert_eq!(
            sut.per_factor
                .get(&i.factor_source_id())
                .unwrap()
                .to_use_directly,
            FactorInstances::just(i)
        )
    }

    #[test]
    fn only_found_in_cache() {
        let i = HierarchicalDeterministicFactorInstance::fia0();

        let sut = Sut::transpose(
            IndexMap::new(),
            IndexMap::new(),
            IndexMap::kv(
                FactorSourceIDFromHash::sample_at(0),
                FactorInstances::just(i.clone()),
            ),
            IndexMap::new(),
        );
        assert_eq!(
            sut.per_factor
                .get(&i.factor_source_id())
                .unwrap()
                .found_in_cache,
            FactorInstances::just(i)
        )
    }

    #[test]
    fn only_newly_derived() {
        let i = HierarchicalDeterministicFactorInstance::fia0();

        let sut = Sut::transpose(
            IndexMap::new(),
            IndexMap::new(),
            IndexMap::new(),
            IndexMap::kv(
                FactorSourceIDFromHash::sample_at(0),
                FactorInstances::just(i.clone()),
            ),
        );
        assert_eq!(
            sut.per_factor
                .get(&i.factor_source_id())
                .unwrap()
                .newly_derived,
            FactorInstances::just(i)
        )
    }
}
 */
