use crate::prelude::*;

#[uniffi::export]
pub fn new_profile_network_sample() -> ProfileNetwork {
    ProfileNetwork::sample()
}

#[uniffi::export]
pub fn new_profile_network_sample_other() -> ProfileNetwork {
    ProfileNetwork::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ProfileNetwork;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_profile_network_sample(),
                new_profile_network_sample_other(),
                // duplicates should get removed
                new_profile_network_sample(),
                new_profile_network_sample_other(),
            ])
            .len(),
            2
        );
    }
}
