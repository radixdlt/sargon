use crate::prelude::*;
use sargon::SecurityStructureMetadata as InternalSecurityStructureMetadata;

#[derive(
    Clone, Debug, PartialEq, Eq, Hash,  uniffi::Record,
)]
pub struct SecurityStructureMetadata {
    pub id: SecurityStructureID,
    pub display_name: DisplayName,
    pub created_on: Timestamp,
    pub last_updated_on: Timestamp,
}

impl From<InternalSecurityStructureMetadata> for SecurityStructureMetadata {
    fn from(value: InternalSecurityStructureMetadata) -> Self {
        Self {
            id: value.id.into(),
            display_name: value.display_name.into(),
            created_on: value.created_on.into(),
            last_updated_on: value.last_updated_on.into(),
        }
    }
}

impl Into<InternalSecurityStructureMetadata> for SecurityStructureMetadata {
    fn into(self) -> InternalSecurityStructureMetadata {
        InternalSecurityStructureMetadata {
            id: self.id.into(),
            display_name: self.display_name.into(),
            created_on: self.created_on.into(),
            last_updated_on: self.last_updated_on.into(),
        }
    }
}

#[uniffi::export]
pub fn new_security_structure_metadata_sample() -> SecurityStructureMetadata {
    InternalSecurityStructureMetadata::sample()
}

#[uniffi::export]
pub fn new_security_structure_metadata_sample_other(
) -> SecurityStructureMetadata {
    InternalSecurityStructureMetadata::sample_other()
}

#[uniffi::export]
pub fn new_security_structure_metadata_named(
    name: &DisplayName,
) -> SecurityStructureMetadata {
    InternalSecurityStructureMetadata::new(name.into())
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
