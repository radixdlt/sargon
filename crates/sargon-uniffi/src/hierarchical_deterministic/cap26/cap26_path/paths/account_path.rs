use crate::prelude::*;
use sargon::AccountPath as InternalAccountPath;
use sargon::EntityCAP26Path;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct AccountPath {
    pub path: HDPath,

    pub network_id: NetworkID,

    pub entity_kind: CAP26EntityKind,

    pub key_kind: CAP26KeyKind,

    pub index: HDPathValue,
}

#[uniffi::export]
pub fn new_account_path_sample() -> AccountPath {
    InternalAccountPath::sample().into()
}

#[uniffi::export]
pub fn new_account_path_sample_other() -> AccountPath {
    InternalAccountPath::sample_other().into()
}

#[uniffi::export]
pub fn new_account_path(
    network_id: NetworkID,
    key_kind: CAP26KeyKind,
    index: HDPathValue,
) -> AccountPath {
    InternalAccountPath::new(network_id.into(), key_kind.into(), index.into())
        .into()
}
