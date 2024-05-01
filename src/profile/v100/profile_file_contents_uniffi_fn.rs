use crate::prelude::*;
use std::hash::{DefaultHasher, Hash, Hasher};

#[uniffi::export]
pub fn profile_file_contents_equals(
    lhs: &ProfileFileContents,
    rhs: &ProfileFileContents,
) -> bool {
    lhs == rhs
}

#[uniffi::export]
pub fn profile_file_contents_hash_value(contents: &ProfileFileContents) -> u64 {
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
}
