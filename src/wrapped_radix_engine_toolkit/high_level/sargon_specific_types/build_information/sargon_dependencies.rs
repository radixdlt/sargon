use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct SargonDependencies {
    pub radix_engine_toolkit: DependencyInformation,
    pub scrypto_radix_engine: DependencyInformation,
}

impl SargonDependencies {
    pub fn get() -> Self {
        Self {
            radix_engine_toolkit: DependencyInformation::with_value(env!(
                "RADIX-ENGINE-TOOLKIT_DEPENDENCY"
            )),
            scrypto_radix_engine: DependencyInformation::with_value(env!(
                "RADIX-ENGINE_DEPENDENCY"
            )),
        }
    }
}

impl HasSampleValues for SargonDependencies {
    fn sample() -> Self {
        Self {
            radix_engine_toolkit: DependencyInformation::sample(),
            scrypto_radix_engine: DependencyInformation::sample_other(),
        }
    }
    fn sample_other() -> Self {
        Self {
            radix_engine_toolkit: DependencyInformation::sample_other(),
            scrypto_radix_engine: DependencyInformation::sample(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonDependencies;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    #[ignore]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
