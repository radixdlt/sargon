use crate::prelude::*;

#[uniffi::export]
pub fn new_profile_file_contents_sample() -> ProfileFileContents {
    ProfileFileContents::sample()
}

#[uniffi::export]
pub fn new_profile_file_contents_sample_other() -> ProfileFileContents {
    ProfileFileContents::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ProfileFileContents;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_profile_file_contents_sample(),
                new_profile_file_contents_sample_other(),
                // duplicates should get removed
                new_profile_file_contents_sample(),
                new_profile_file_contents_sample_other(),
            ])
            .len(),
            2
        );
    }
}
