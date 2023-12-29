use std::sync::Arc;

use crate::HDPathError;
use crate::{
    AccountPath, CAP26Repr, Derivation, DerivationPath, DerivationPathScheme, GetIDPath, HDPath,
    IdentityPath,
};
use enum_as_inner::EnumAsInner;
use serde::{de, Deserializer, Serialize, Serializer};

#[cfg(any(test, feature = "placeholder"))]
use crate::HasPlaceholder;

/// A derivation path design specifically for Radix Babylon wallets used by Accounts and Personas
/// to be unique per network with separate key spaces for Accounts/Identities (Personas) and key
/// kind: sign transaction or sign auth.
#[derive(Clone, Debug, PartialEq, EnumAsInner, Eq, Hash, PartialOrd, Ord, uniffi::Enum)]
pub enum CAP26Path {
    GetID { value: Arc<GetIDPath> },
    AccountPath { value: AccountPath },
    IdentityPath { value: IdentityPath },
}

impl TryFrom<&HDPath> for CAP26Path {
    type Error = HDPathError;

    fn try_from(value: &HDPath) -> Result<Self, Self::Error> {
        if let Ok(get_id) = GetIDPath::try_from(value) {
            return Ok(get_id.into());
        }
        if let Ok(identity_path) = IdentityPath::try_from(value) {
            return Ok(identity_path.into());
        }
        return AccountPath::try_from(value).map(|p| p.into());
    }
}

impl Serialize for CAP26Path {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for CAP26Path {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<CAP26Path, D::Error> {
        let s = String::deserialize(d)?;
        if let Ok(getid) = GetIDPath::from_str(&s) {
            return Ok(CAP26Path::GetID {
                value: getid.into(),
            });
        } else {
            AccountPath::from_str(&s)
                .map(|value| Self::AccountPath { value })
                .map_err(de::Error::custom)
        }
    }
}

impl Derivation for CAP26Path {
    fn hd_path(&self) -> &HDPath {
        match self {
            CAP26Path::AccountPath { value } => value.hd_path(),
            CAP26Path::IdentityPath { value } => value.hd_path(),
            CAP26Path::GetID { value } => value.hd_path(),
        }
    }

    fn derivation_path(&self) -> DerivationPath {
        DerivationPath::CAP26 {
            value: self.clone(),
        }
    }

    fn scheme(&self) -> DerivationPathScheme {
        match self {
            CAP26Path::AccountPath { value } => value.scheme(),
            CAP26Path::IdentityPath { value } => value.scheme(),
            CAP26Path::GetID { value } => value.scheme(),
        }
    }
}

impl From<AccountPath> for CAP26Path {
    fn from(value: AccountPath) -> Self {
        Self::AccountPath { value }
    }
}
impl From<IdentityPath> for CAP26Path {
    fn from(value: IdentityPath) -> Self {
        Self::IdentityPath { value }
    }
}
impl From<GetIDPath> for CAP26Path {
    fn from(value: GetIDPath) -> Self {
        Self::GetID {
            value: value.into(),
        }
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl HasPlaceholder for CAP26Path {
    fn placeholder() -> Self {
        Self::placeholder_account()
    }
    fn placeholder_other() -> Self {
        Self::placeholder_identity()
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl CAP26Path {
    pub fn placeholder_account() -> Self {
        Self::AccountPath {
            value: AccountPath::placeholder(),
        }
    }

    pub fn placeholder_identity() -> Self {
        Self::IdentityPath {
            value: IdentityPath::placeholder(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_json_value_eq_after_roundtrip, HasPlaceholder};
    use serde_json::json;

    use crate::{AccountPath, Derivation, DerivationPathScheme, GetIDPath};

    use super::CAP26Path;

    #[test]
    fn equality() {
        assert_eq!(CAP26Path::placeholder(), CAP26Path::placeholder());
        assert_eq!(
            CAP26Path::placeholder_other(),
            CAP26Path::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(CAP26Path::placeholder(), CAP26Path::placeholder_other());
    }

    #[test]
    fn scheme_account_path() {
        assert_eq!(
            CAP26Path::placeholder_account().scheme(),
            DerivationPathScheme::Cap26
        );
    }

    #[test]
    fn scheme_identity_path() {
        assert_eq!(
            CAP26Path::placeholder_identity().scheme(),
            DerivationPathScheme::Cap26
        );
    }

    #[test]
    fn scheme_getid_path() {
        assert_eq!(
            CAP26Path::GetID {
                value: GetIDPath::default().into()
            }
            .scheme(),
            DerivationPathScheme::Cap26
        );
    }

    #[test]
    fn hdpath_account_path() {
        assert_eq!(
            CAP26Path::placeholder_account().hd_path(),
            AccountPath::placeholder().hd_path()
        );
    }

    #[test]
    fn hdpath_getid_path() {
        assert_eq!(
            CAP26Path::GetID {
                value: GetIDPath::default().into()
            }
            .hd_path(),
            GetIDPath::default().hd_path()
        );
    }

    #[test]
    fn into_from_account_path() {
        assert_eq!(
            CAP26Path::AccountPath {
                value: AccountPath::placeholder()
            },
            AccountPath::placeholder().into()
        );
    }

    #[test]
    fn into_from_getid_path() {
        assert_eq!(
            CAP26Path::GetID {
                value: GetIDPath::default().into()
            },
            GetIDPath::default().into()
        );
    }

    #[test]
    fn json_roundtrip_getid() {
        let model: CAP26Path = GetIDPath::default().into();
        assert_json_value_eq_after_roundtrip(&model, json!("m/44H/1022H/365H"));
    }

    #[test]
    fn json_roundtrip_account() {
        let model: CAP26Path = AccountPath::placeholder().into();
        assert_json_value_eq_after_roundtrip(&model, json!("m/44H/1022H/1H/525H/1460H/0H"));
    }
}
