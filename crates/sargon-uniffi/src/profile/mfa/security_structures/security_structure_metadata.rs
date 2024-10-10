use crate::prelude::*;
use sargon::SecurityStructureMetadata as InternalSecurityStructureMetadata;

#[derive(
    Clone,  PartialEq, Eq, Hash,  uniffi::Record,
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
            id: self.id.into_internal(),
            display_name: self.display_name.into_internal(),
            created_on: self.created_on.into_internal(),
            last_updated_on: self.last_updated_on.into_internal(),
        }
    }
}

#[uniffi::export]
pub fn new_security_structure_metadata_sample() -> SecurityStructureMetadata {
    InternalSecurityStructureMetadata::sample().into()
}

#[uniffi::export]
pub fn new_security_structure_metadata_sample_other(
) -> SecurityStructureMetadata {
    InternalSecurityStructureMetadata::sample_other().into()
}

#[uniffi::export]
pub fn new_security_structure_metadata_named(
    name: &DisplayName,
) -> SecurityStructureMetadata {
    InternalSecurityStructureMetadata::new(&name.into_internal()).into()
}

