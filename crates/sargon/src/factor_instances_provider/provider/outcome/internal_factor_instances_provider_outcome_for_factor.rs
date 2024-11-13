use crate::prelude::*;

#[derive(Clone, derive_more::Debug)]
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

    /// FactorInstances which are saved into the cache
    ///
    /// Might be empty
    pub to_cache: FactorInstances,

    /// FactorInstances which are not saved into the cache.
    ///
    /// Might be empty
    pub to_use_directly: FactorInstances,

    /// FactorInstances which was found in the cache before the operation was
    /// executed.
    ///
    /// Might be empty
    ///
    /// Useful for unit tests.
    ///
    /// Might overlap with `to_use_directly`
    pub found_in_cache: FactorInstances,

    /// FactorInstances which was newly derived.
    ///
    /// Might be empty
    ///
    /// Useful for unit tests.
    ///
    /// Might overlap with `to_cache` and `to_use_directly`
    pub newly_derived: FactorInstances,
}

impl InternalFactorInstancesProviderOutcomeForFactor {
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
