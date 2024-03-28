use crate::prelude::*;

#[uniffi::export]
pub fn new_dependency_information_sample() -> DependencyInformation {
    DependencyInformation::sample()
}

#[uniffi::export]
pub fn new_dependency_information_sample_other() -> DependencyInformation {
    DependencyInformation::sample_other()
}


#[uniffi::export]
pub fn dependency_information_to_string(info: &DependencyInformation) -> String {
    info.to_string()
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
