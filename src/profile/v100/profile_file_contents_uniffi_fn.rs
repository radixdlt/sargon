use crate::prelude::*;
use std::hash::{DefaultHasher, Hash, Hasher};

#[uniffi::export]
pub(crate) fn new_profile_file_contents_sample() -> ProfileFileContents {
    ProfileFileContents::sample()
}

#[uniffi::export]
pub(crate) fn new_profile_file_contents_sample_other() -> ProfileFileContents {
    ProfileFileContents::sample_other()
}

#[uniffi::export]
pub(crate) fn profile_file_contents_equals(
    lhs: &ProfileFileContents,
    rhs: &ProfileFileContents,
) -> bool {
    lhs == rhs
}

#[uniffi::export]
pub(crate) fn profile_file_contents_hash_value(
    contents: &ProfileFileContents,
) -> u64 {
    let mut hasher = DefaultHasher::new();
    contents.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ProfileFileContents;

    #[test]
    fn test_equals() {
        assert!(profile_file_contents_equals(&SUT::sample(), &SUT::sample()));
        assert!(!profile_file_contents_equals(
            &SUT::sample(),
            &SUT::sample_other()
        ));
    }

    #[test]
    fn test_hash_value() {
        assert_eq!(
            profile_file_contents_hash_value(&SUT::sample()),
            profile_file_contents_hash_value(&SUT::sample()),
        );
        assert_eq!(
            profile_file_contents_hash_value(&SUT::sample_other()),
            profile_file_contents_hash_value(&SUT::sample_other()),
        );
        assert_ne!(
            profile_file_contents_hash_value(&SUT::sample()),
            profile_file_contents_hash_value(&SUT::sample_other()),
        )
    }

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_profile_file_contents_sample(),
                new_profile_file_contents_sample_other(),
                SUT::NotProfile,
                // duplicates should get removed
                new_profile_file_contents_sample(),
                new_profile_file_contents_sample_other(),
                SUT::NotProfile,
            ])
            .len(),
            3
        );
    }
}
