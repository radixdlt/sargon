use crate::prelude::*;
use sargon::IdentityPath as InternalIdentityPath;
use sargon::NewEntityPath;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct IdentityPath {
    pub network_id: NetworkID,
    pub key_kind: CAP26KeyKind,
    pub index: Hardened,
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
    index: Hardened,
) -> IdentityPath {
    InternalIdentityPath::new(network_id, key_kind, index).into()
}
