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
    #[display("{value}")]
    GetID { value: GetIDPath },
    #[display("{value}")]
    Account { value: AccountPath },
    #[display("{value}")]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_path() {
        let path = CAP26Path::sample_account();
        assert_eq!(
            path,
            new_cap26_path_from_string(cap26_path_to_string(&path)).unwrap()
        );
    }

    #[test]
    fn test_identity_path() {
        let path = CAP26Path::sample_identity();
        assert_eq!(
            path,
            new_cap26_path_from_string(cap26_path_to_string(&path)).unwrap()
        );
    }

    #[test]
    fn test_get_id_path() {
        let path = CAP26Path::from(default_get_id_path());
        assert_eq!(
            path,
            new_cap26_path_from_string(String::from("m/44H/1022H/365H"))
                .unwrap()
        );
    }

    #[test]
    fn test_new_account_path() {
        let path = new_account_path(
            NetworkID::Mainnet,
            CAP26KeyKind::TransactionSigning,
            0,
        );
        assert_eq!(path.to_string(), "m/44H/1022H/1H/525H/1460H/0H")
    }

    #[test]
    fn test_new_identity_path() {
        let path = new_identity_path(
            NetworkID::Mainnet,
            CAP26KeyKind::TransactionSigning,
            0,
        );
        assert_eq!(path.to_string(), "m/44H/1022H/1H/618H/1460H/0H")
    }
}
