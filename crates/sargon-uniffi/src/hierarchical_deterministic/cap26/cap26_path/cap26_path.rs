use crate::prelude::*;
use sargon::CAP26Path as InternalCAP26Path;

/// A derivation path design specifically for Radix Babylon wallets used by Accounts and Personas
/// to be unique per network with separate key spaces for Accounts/Identities (Personas) and key
/// kind: sign transaction or sign auth.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
pub enum CAP26Path {
    GetID { value: GetIDPath },
    Account { value: AccountPath },
    Identity { value: IdentityPath },
}

#[uniffi::export]
pub fn new_cap26_path_from_string(string: String) -> Result<CAP26Path> {
    InternalCAP26Path::from_str(&string).map_result()
}

#[uniffi::export]
pub fn cap26_path_to_string(path: &CAP26Path) -> String {
    path.into_internal().to_string()
}
