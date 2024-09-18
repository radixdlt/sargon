use crate::prelude::*;

uniffi::custom_newtype!(ProfileID, Uuid);

#[uniffi::export]
pub fn new_profile_id_sample() -> ProfileID {
    ProfileID::sample()
}

#[uniffi::export]
pub fn new_profile_id_sample_other() -> ProfileID {
    ProfileID::sample_other()
}

#[cfg(test)]
mod uniffi_test {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ProfileID;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_profile_id_sample(),
                new_profile_id_sample_other(),
                // duplicates should get removed
                new_profile_id_sample(),
                new_profile_id_sample_other(),
            ])
            .len(),
            2
        );
    }
}
