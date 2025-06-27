use crate::prelude::*;
use sargon::SecurityStructureFlag as InternalSecurityStructureFlag;
use sargon::SecurityStructureFlags as InternalSecurityStructureFlags;
use sargon::SecurityStructureMetadata as InternalSecurityStructureMetadata;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct SecurityStructureMetadata {
    pub id: SecurityStructureID,
    pub display_name: DisplayName,
    pub created_on: Timestamp,
    pub last_updated_on: Timestamp,
    pub flags: Vec<SecurityStructureFlag>,
}

delegate_debug_into!(
    SecurityStructureMetadata,
    InternalSecurityStructureMetadata
);

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
    InternalSecurityStructureMetadata::new(
        name.into_internal(),
        InternalSecurityStructureFlags::just(
            InternalSecurityStructureFlag::Main,
        ),
    )
    .into()
}

#[uniffi::export]
pub fn security_structure_metadata_is_main(
    security_structure_metadata: &SecurityStructureMetadata,
) -> bool {
    security_structure_metadata.into_internal().is_main()
}
