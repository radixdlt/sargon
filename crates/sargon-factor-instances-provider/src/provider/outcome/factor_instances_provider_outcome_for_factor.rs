use crate::prelude::*;

/// Identical to `InternalFactorInstancesProviderOutcomeForFactor` but with
/// different field names, making it clear that the instances of `to_cache` field in the
/// "non-final" counterpart has already been cached, thus here named
/// `debug_was_cached`.
/// Furthermore all fields except `to_use_directly` are renamed to `debug_*` to make it clear they are only included for debugging purposes,
/// in fact, they are all put behind `#[cfg(test)]`
#[derive(Clone, derive_more::Debug, PartialEq, Eq)]
#[debug("{}", self.debug_string())]
pub struct FactorInstancesProviderOutcomeForFactor {
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

    /// FactorInstances which were saved into the cache
    ///
    /// Might be empty
    ///
    /// Useful for unit tests.
    pub debug_was_cached: FactorInstances,

    /// FactorInstances which were found in the cache before the operation was
    /// executed.
    ///
    /// Might be empty
    ///
    /// Useful for unit tests.
    ///
    /// Might overlap with `to_use_directly`
    pub debug_found_in_cache: FactorInstances,

    /// FactorInstances which were derived.
    ///
    /// Might be empty
    ///
    /// Useful for unit tests.
    ///
    /// Might overlap with `debug_was_cached` and `to_use_directly`
    pub debug_was_derived: FactorInstances,
}

#[allow(dead_code)]
impl FactorInstancesProviderOutcomeForFactor {
    #[cfg(test)]
    fn debug_string_for_tests(&self) -> String {
        format!(
            "OutcomeForFactor[factor: {}\n\n\t⚡️to_use_directly: {:?}, \n\n\t➡️💾was_cached: {:?}, \n\n\t💾➡️found_in_cache: {:?}\n\n\t🔮was_derived: {:?}\n\n]",
            self.factor_source_id, self.to_use_directly, self.debug_was_cached, self.debug_found_in_cache, self.debug_was_derived
        )
    }

    fn debug_string_no_test(&self) -> String {
        format!(
            "OutcomeForFactor[factor: {}, \n\n\t⚡️to_use_directly: {:?}]",
            self.factor_source_id, self.to_use_directly
        )
    }

    fn debug_string(&self) -> String {
        #[cfg(test)]
        return self.debug_string_for_tests();

        #[cfg(not(test))]
        return self.debug_string_no_test();
    }
}

impl From<InternalFactorInstancesProviderOutcomeForFactor>
    for FactorInstancesProviderOutcomeForFactor
{
    fn from(value: InternalFactorInstancesProviderOutcomeForFactor) -> Self {
        let _self = Self {
            hidden: HiddenConstructor,
            factor_source_id: value.factor_source_id,
            to_use_directly: value.to_use_directly,
            debug_was_cached: value.to_cache,
            debug_found_in_cache: value.found_in_cache,
            debug_was_derived: value.newly_derived,
        };

        _self
    }
}

impl HasSampleValues for FactorInstancesProviderOutcomeForFactor {
    fn sample() -> Self {
        Self::from(InternalFactorInstancesProviderOutcomeForFactor::sample())
    }

    fn sample_other() -> Self {
        Self::from(
            InternalFactorInstancesProviderOutcomeForFactor::sample_other(),
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorInstancesProviderOutcomeForFactor;

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
    fn debug_string_for_tests() {
        assert_eq!(
            SUT::sample().debug_string_for_tests(),
            SUT::sample().debug_string_for_tests()
        );
        assert_eq!(
            SUT::sample_other().debug_string_for_tests(),
            SUT::sample_other().debug_string_for_tests()
        );
        assert_ne!(
            SUT::sample().debug_string_for_tests(),
            SUT::sample_other().debug_string_for_tests()
        );
    }

    #[test]
    fn debug_string_no_test() {
        assert_eq!(
            SUT::sample().debug_string_no_test(),
            SUT::sample().debug_string_no_test()
        );
        assert_eq!(
            SUT::sample_other().debug_string_no_test(),
            SUT::sample_other().debug_string_no_test()
        );
        assert_ne!(
            SUT::sample().debug_string_no_test(),
            SUT::sample_other().debug_string_no_test()
        );
    }
}
