use crate::prelude::*;
use sargon::SargonDependencies as InternalSargonDependencies;

#[derive(Clone, Debug, PartialEq, Eq, Hash,  uniffi::Record)]
pub struct SargonDependencies {
    pub radix_engine_toolkit: DependencyInformation,
    pub scrypto_radix_engine: DependencyInformation,
}

impl From<InternalSargonDependencies> for SargonDependencies {
    fn from(value: InternalSargonDependencies) -> Self {
        Self {
            radix_engine_toolkit: value.radix_engine_toolkit.into(),
            scrypto_radix_engine: value.scrypto_radix_engine.into(),
        }
    }
}

impl Into<InternalSargonDependencies> for SargonDependencies {
    fn into(self) -> InternalSargonDependencies {
        InternalSargonDependencies {
            radix_engine_toolkit: self.radix_engine_toolkit.into(),
            scrypto_radix_engine: self.scrypto_radix_engine.into(),
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
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
