use crate::prelude::*;
use sargon::{AccountPath as InternalAccountPath, NewEntityPath};

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct AccountPath {
    pub network_id: NetworkID,
    pub key_kind: CAP26KeyKind,
    pub index: Hardened,
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
    index: Hardened,
) -> AccountPath {
    InternalAccountPath::new(network_id, key_kind, index).into()
}
