use crate::prelude::*;
use sargon::CAP26Path as InternalCAP26Path;

/// A derivation path design specifically for Radix Babylon wallets used by Accounts and Personas
/// to be unique per network with separate key spaces for Accounts/Identities (Personas) and key
/// kind: sign transaction or sign auth.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    uniffi::Enum,
)]
pub enum CAP26Path {
    GetID { value: GetIDPath },
    Account { value: AccountPath },
    Identity { value: IdentityPath },
}

impl From<InternalCAP26Path> for CAP26Path {
    fn from(value: InternalCAP26Path) -> Self {
        match value {
            InternalCAP26Path::GetID(value) => Self::GetID {
                value: value.into(),
            },
            InternalCAP26Path::Account(value) => Self::Account {
                value: value.into(),
            },
            InternalCAP26Path::Identity(value) => Self::Identity {
                value: value.into(),
            },
        }
    }
}

impl Into<InternalCAP26Path> for CAP26Path {
    fn into(self) -> InternalCAP26Path {
        match self {
            Self::GetID { value } => InternalCAP26Path::GetID(value.into()),
            Self::Account { value } => InternalCAP26Path::Account(value.into()),
            Self::Identity { value } => InternalCAP26Path::Identity(value.into()),
        }
    }
}

#[uniffi::export]
pub fn new_cap26_path_from_string(
    string: String,
) -> Result<CAP26Path> {
    map_result_from_internal(InternalCAP26Path::from_str(&string))
}

#[uniffi::export]
pub fn default_get_id_path() -> GetIDPath {
    GetIDPath::default()
}

#[uniffi::export]
pub fn cap26_path_to_string(path: &CAP26Path) -> String {
    path.into::<InternalCAP26Path>().to_string()
}

