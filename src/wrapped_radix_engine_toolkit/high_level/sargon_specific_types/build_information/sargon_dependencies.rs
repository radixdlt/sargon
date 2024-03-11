use crate::prelude::*;

pub(crate) const RADIX_ENGINE_TOOLKIT_DEPENDENCY: &str =
    "RADIX-ENGINE-TOOLKIT-DEPENDENCY";
pub(crate) const RADIX_ENGINE_DEPENDENCY: &str = "RADIX-ENGINE-DEPENDENCY";

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct SargonDependencies {
    pub radix_engine_toolkit: DependencyInformation,
    pub scrypto_radix_engine: DependencyInformation,
}

impl SargonDependencies {
    pub fn get() -> Self {
        Self {
            radix_engine_toolkit: DependencyInformation::of(
                RADIX_ENGINE_TOOLKIT_DEPENDENCY,
            ),
            scrypto_radix_engine: DependencyInformation::of(
                RADIX_ENGINE_DEPENDENCY,
            ),
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
        let ret_v = "0.0.1";
        let re_rev =
            "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef";
        std::env::set_var(
            RADIX_ENGINE_TOOLKIT_DEPENDENCY,
            format!("version = {}", ret_v),
        );
        std::env::set_var(RADIX_ENGINE_DEPENDENCY, format!("rev = {}", re_rev));
        let val = Self::get();
        std::env::remove_var(RADIX_ENGINE_TOOLKIT_DEPENDENCY);
        std::env::remove_var(RADIX_ENGINE_DEPENDENCY);
        val
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
