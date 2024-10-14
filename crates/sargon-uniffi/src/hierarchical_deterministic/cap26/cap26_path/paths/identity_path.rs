use crate::prelude::*;
use sargon::EntityCAP26Path;
use sargon::IdentityPath as InternalIdentityPath;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Record)]
pub struct IdentityPath {
    pub path: HDPath,

    pub network_id: NetworkID,

    pub entity_kind: CAP26EntityKind,

    pub key_kind: CAP26KeyKind,

    pub index: HDPathValue,
}

#[uniffi::export]
pub fn new_identity_path_sample() -> IdentityPath {
    InternalIdentityPath::sample().into()
}

#[uniffi::export]
pub fn new_identity_path_sample_other() -> IdentityPath {
    InternalIdentityPath::sample_other().into()
}

#[uniffi::export]
pub fn new_identity_path(
    network_id: NetworkID,
    key_kind: CAP26KeyKind,
    index: HDPathValue,
) -> IdentityPath {
    InternalIdentityPath::new(network_id.into(), key_kind.into(), index.into())
        .into()
}
