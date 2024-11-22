use crate::prelude::*;

#[derive(Clone, derive_more::Debug, PartialEq, Eq)]
#[debug(
    "InternalFactorInstancesProviderOutcomeForFactor[ factor: {:?}\n\n\t⚡️ to_use_directly: {:?}\n\n\t➡️💾to_cache: {:?}\n\n\t💾➡️found_in_cache: {:?}\n\n\t🔮derived: {:?}\n\n]\n",
    factor_source_id,
    to_use_directly,
    to_cache,
    found_in_cache,
    newly_derived
)]
pub struct InternalFactorInstancesProviderOutcomeForFactor {
    #[allow(dead_code)]
    #[doc(hidden)]
    #[debug(skip)]
    hidden: HiddenConstructor,

    /// The FactorSourceID of all the factor instances of this type.
    pub factor_source_id: FactorSourceIDFromHash,

    /// FactorInstances which are not saved into the cache.
    ///
    /// Might be empty
    pub to_use_directly: FactorInstances,

    /// FactorInstances which are saved into the cache
    ///
    /// Might be empty
    pub to_cache: FactorInstances,

    /// FactorInstances which were found in the cache before the operation was
    /// executed.
    ///
    /// Might be empty
    ///
    /// Useful for unit tests.
    ///
    /// Might overlap with `to_use_directly`
    pub found_in_cache: FactorInstances,

    /// FactorInstances which were newly derived.
    ///
    /// Might be empty
    ///
    /// Useful for unit tests.
    ///
    /// Might overlap with `to_cache` and `to_use_directly`
    pub newly_derived: FactorInstances,
}

impl InternalFactorInstancesProviderOutcomeForFactor {
    /// # Panics
    /// Panics if not all FactorInstances were derived from `factor_source_id`
    pub fn new(
        factor_source_id: FactorSourceIDFromHash,
        to_cache: FactorInstances,
        to_use_directly: FactorInstances,
        found_in_cache: FactorInstances,
        newly_derived: FactorInstances,
    ) -> Self {
        let assert_factor = |xs: &FactorInstances| {
            assert!(
                xs.factor_instances()
                    .iter()
                    .all(|x| x.factor_source_id() == factor_source_id),
                "Discrepancy factor source id"
            );
        };
        assert_factor(&to_cache);
        assert_factor(&to_use_directly);
        assert_factor(&found_in_cache);
        assert_factor(&newly_derived);

        Self {
            hidden: HiddenConstructor,
            factor_source_id,
            to_cache,
            to_use_directly,
            found_in_cache,
            newly_derived,
        }
    }

    pub fn satisfied_by_cache(
        factor_source_id: FactorSourceIDFromHash,
        found_in_cache: FactorInstances,
    ) -> Self {
        let to_use_directly = found_in_cache.clone();

        // nothing to cache
        let to_cache = FactorInstances::default();

        // nothing was derived
        let newly_derived = FactorInstances::default();

        Self::new(
            factor_source_id,
            to_cache,
            to_use_directly,
            found_in_cache,
            newly_derived,
        )
    }
}

impl HasSampleValues for InternalFactorInstancesProviderOutcomeForFactor {
    fn sample() -> Self {
        Self::new(FactorSourceIDFromHash::sample_at(0),    FactorInstances::new(IndexSet::from_iter([
    HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(2),
])),    FactorInstances::new(IndexSet::from_iter([
    HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(0),
])),    FactorInstances::new(IndexSet::from_iter([
    HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(0),
])),    FactorInstances::new(IndexSet::from_iter([
    HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(1),
    HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(2),
])))
    }

    fn sample_other() -> Self {
        Self::new(FactorSourceIDFromHash::sample_at(1),    FactorInstances::new(IndexSet::from_iter([
            HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_1_securified_at_index(2),
        ])),    FactorInstances::new(IndexSet::from_iter([
            HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_1_securified_at_index(0),
        ])),    FactorInstances::new(IndexSet::from_iter([
            HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_1_securified_at_index(0),
        ])),    FactorInstances::new(IndexSet::from_iter([
            HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_1_securified_at_index(1),
            HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_1_securified_at_index(2),
        ])))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = InternalFactorInstancesProviderOutcomeForFactor;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    #[should_panic]
    fn wrong_factor_source_id_to_cache() {
        _ = SUT::new(
            FactorSourceIDFromHash::sample_at(1),
            FactorInstances::sample(),
            FactorInstances::default(),
            FactorInstances::default(),
            FactorInstances::default(),
        );
    }

    #[test]
    #[should_panic]
    fn wrong_factor_source_id_to_use_dir() {
        _ = SUT::new(
            FactorSourceIDFromHash::sample_at(1),
            FactorInstances::default(),
            FactorInstances::sample(),
            FactorInstances::default(),
            FactorInstances::default(),
        );
    }

    #[test]
    #[should_panic]
    fn wrong_factor_source_id_found_in_cache() {
        _ = SUT::new(
            FactorSourceIDFromHash::sample_at(1),
            FactorInstances::default(),
            FactorInstances::default(),
            FactorInstances::sample(),
            FactorInstances::default(),
        );
    }

    #[test]
    #[should_panic]
    fn wrong_factor_source_id_newly_derived() {
        _ = SUT::new(
            FactorSourceIDFromHash::sample_at(1),
            FactorInstances::default(),
            FactorInstances::default(),
            FactorInstances::default(),
            FactorInstances::sample(),
        );
    }

    #[test]
    fn debug_string() {
        assert_eq!(
            format!("{:?}", SUT::sample()),
            format!("{:?}", SUT::sample())
        );

        assert_eq!(
            format!("{:?}", SUT::sample_other()),
            format!("{:?}", SUT::sample_other())
        );

        assert_ne!(
            format!("{:?}", SUT::sample()),
            format!("{:?}", SUT::sample_other())
        );
    }
}
