use std::sync::Once;

use crate::prelude::*;

#[uniffi::export]
pub fn new_security_structure_metadata_sample() -> SecurityStructureMetadata {
    SecurityStructureMetadata::sample()
}

#[uniffi::export]
pub fn new_security_structure_metadata_sample_other(
) -> SecurityStructureMetadata {
    SecurityStructureMetadata::sample_other()
}

#[uniffi::export]
pub fn new_security_structure_metadata_named(
    name: &DisplayName,
) -> SecurityStructureMetadata {
    SecurityStructureMetadata::new(name.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityStructureMetadata;

    #[test]
    fn test_new() {
        let name = &DisplayName::sample();
        assert_ne!(
            new_security_structure_metadata_named(name),
            new_security_structure_metadata_named(name)
        )
    }

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_security_structure_metadata_sample(),
                new_security_structure_metadata_sample_other(),
                // duplicates should get removed
                new_security_structure_metadata_sample(),
                new_security_structure_metadata_sample_other(),
            ])
            .len(),
            2
        );
    }
}
