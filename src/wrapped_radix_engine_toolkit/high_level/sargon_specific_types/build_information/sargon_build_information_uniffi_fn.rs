use std::sync::Once;

use crate::prelude::*;

#[uniffi::export]
pub fn new_sargon_build_information_sample() -> SargonBuildInformation {
    SargonBuildInformation::sample()
}

#[uniffi::export]
pub fn new_sargon_build_information_sample_other() -> SargonBuildInformation {
    SargonBuildInformation::sample_other()
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
