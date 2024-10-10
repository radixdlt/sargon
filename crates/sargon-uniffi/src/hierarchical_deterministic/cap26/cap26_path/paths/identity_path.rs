use crate::prelude::*;
use sargon::IdentityPath as InternalIdentityPath;

#[derive(
    Clone,
    
    PartialEq,
    Eq,
    Hash,
     uniffi::Record,
)]
pub struct IdentityPath {
    pub path: HDPath,

    pub network_id: NetworkID,

    pub entity_kind: CAP26EntityKind,

    pub key_kind: CAP26KeyKind,

    pub index: HDPathValue,
}

impl From<InternalIdentityPath> for IdentityPath {
    fn from(value: InternalIdentityPath) -> Self {
        Self {
            path: value.path.into(),
            network_id: value.network_id.into(),
            entity_kind: value.entity_kind.into(),
            key_kind: value.key_kind.into(),
            index: value.index.into(),
        }
    }
}

impl Into<InternalIdentityPath> for IdentityPath {
    fn into(self) -> InternalIdentityPath {
        InternalIdentityPath {
            path: self.path.into(),
            network_id: self.network_id.into(),
            entity_kind: self.entity_kind.into(),
            key_kind: self.key_kind.into(),
            index: self.index.into(),
        }
    }
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
    InternalIdentityPath::new(network_id.into(), key_kind.into(), index.into()).into()
}

