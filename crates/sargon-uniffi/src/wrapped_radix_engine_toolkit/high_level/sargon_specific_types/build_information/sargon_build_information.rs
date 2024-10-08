use crate::prelude::*;
use sargon::SargonBuildInformation as InternalSargonBuildInformation;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct SargonBuildInformation {
    pub sargon_version: String,
    pub dependencies: SargonDependencies,
}

impl From<InternalSargonBuildInformation> for SargonBuildInformation {
    fn from(value: InternalSargonBuildInformation) -> Self {
        Self {
            sargon_version: value.sargon_version,
            dependencies: value.dependencies.into(),
        }
    }
}

impl Into<InternalSargonBuildInformation> for SargonBuildInformation {
    fn into(self) -> InternalSargonBuildInformation {
        InternalSargonBuildInformation {
            sargon_version: self.sargon_version,
            dependencies: self.dependencies.into(),
        }
    }
}

#[uniffi::export]
pub fn new_sargon_build_information_sample() -> SargonBuildInformation {
    InternalSargonBuildInformation::sample().into()
}

#[uniffi::export]
pub fn new_sargon_build_information_sample_other() -> SargonBuildInformation {
    InternalSargonBuildInformation::sample_other().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonBuildInformation;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_sargon_build_information_sample(),
                new_sargon_build_information_sample_other(),
                // duplicates should get removed
                new_sargon_build_information_sample(),
                new_sargon_build_information_sample_other(),
            ])
            .len(),
            2
        );
    }
}
