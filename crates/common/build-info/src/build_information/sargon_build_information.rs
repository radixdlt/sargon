use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SargonBuildInformation {
    pub sargon_version: String,
    pub dependencies: SargonDependencies,
}

impl SargonBuildInformation {
    pub fn get() -> Self {
        let sargon_version = env!("CARGO_PKG_VERSION").into();

        Self {
            sargon_version,
            dependencies: SargonDependencies::get(),
        }
    }
}

impl HasSampleValues for SargonBuildInformation {
    fn sample() -> Self {
        Self {
            sargon_version: "0.0.1".to_owned(),
            dependencies: SargonDependencies::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            sargon_version: "0.1.0".to_owned(),
            dependencies: SargonDependencies::sample_other(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonBuildInformation;

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
