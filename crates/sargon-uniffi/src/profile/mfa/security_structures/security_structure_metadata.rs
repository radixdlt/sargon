use crate::prelude::*;

#[derive(
    Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
)]
pub struct SecurityStructureMetadata {
    pub id: SecurityStructureID,
    pub display_name: DisplayName,
    pub created_on: Timestamp,
    pub last_updated_on: Timestamp,
}

impl Identifiable for SecurityStructureMetadata {
    type ID = SecurityStructureID;

    fn id(&self) -> Self::ID {
        self.id
    }
}

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
