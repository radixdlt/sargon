use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

use super::{derivation::Derivation, derivation_path_scheme::DerivationPathScheme};
use crate::{
    bip32::hd_path::HDPath,
    bip44::bip44_like_path::BIP44LikePath,
    cap26::cap26_path::{
        cap26_path::CAP26Path,
        paths::{account_path::AccountPath, getid_path::GetIDPath},
    },
};
use std::fmt::{Debug, Formatter};

/// A derivation path on either supported schemes, either Babylon (CAP26) or Olympia (BIP44Like).
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DerivationPath {
    CAP26(CAP26Path),
    BIP44Like(BIP44LikePath),
}

impl Debug for DerivationPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for DerivationPath {
    /// Tries to deserializes a JSON string as a bech32 address into an `AccountAddress`.
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<DerivationPath, D::Error> {
        #[derive(Deserialize)]
        pub struct Inner {
            pub scheme: DerivationPathScheme,
            pub path: serde_json::Value,
        }

        let inner = Inner::deserialize(d)?;

        let derivation_path = match inner.scheme {
            DerivationPathScheme::Cap26 => DerivationPath::CAP26(
                CAP26Path::deserialize(inner.path).map_err(serde::de::Error::custom)?,
            ),
            DerivationPathScheme::Bip44Olympia => DerivationPath::BIP44Like(
                BIP44LikePath::deserialize(inner.path).map_err(serde::de::Error::custom)?,
            ),
        };
        Ok(derivation_path)
    }
}

impl Serialize for DerivationPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("DerivationPath", 2)?;
        state.serialize_field("scheme", &self.scheme())?;
        state.serialize_field("path", &self.hd_path().to_string())?;
        state.end()
    }
}

impl DerivationPath {
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder() -> Self {
        Self::CAP26(CAP26Path::AccountPath(AccountPath::placeholder()))
    }
}

impl Derivation for DerivationPath {
    fn hd_path(&self) -> &HDPath {
        match self {
            DerivationPath::CAP26(path) => path.hd_path(),
            DerivationPath::BIP44Like(path) => path.hd_path(),
        }
    }
    fn scheme(&self) -> DerivationPathScheme {
        match self {
            DerivationPath::CAP26(p) => p.scheme(),
            DerivationPath::BIP44Like(p) => p.scheme(),
        }
    }
}

impl DerivationPath {
    pub fn placeholder_cap26() -> Self {
        DerivationPath::CAP26(CAP26Path::placeholder_account())
    }
}

impl From<AccountPath> for DerivationPath {
    fn from(value: AccountPath) -> Self {
        Self::CAP26(value.into())
    }
}

impl From<GetIDPath> for DerivationPath {
    fn from(value: GetIDPath) -> Self {
        Self::CAP26(value.into())
    }
}

impl From<BIP44LikePath> for DerivationPath {
    fn from(value: BIP44LikePath) -> Self {
        Self::BIP44Like(value)
    }
}

#[cfg(test)]
mod tests {
    use wallet_kit_common::{json::assert_eq_after_json_roundtrip, network_id::NetworkID};

    use crate::{
        bip44::bip44_like_path::BIP44LikePath,
        cap26::{
            cap26_key_kind::CAP26KeyKind,
            cap26_path::paths::{account_path::AccountPath, getid_path::GetIDPath},
            cap26_repr::CAP26Repr,
        },
        derivation::{derivation::Derivation, derivation_path_scheme::DerivationPathScheme},
    };

    use super::DerivationPath;
    #[test]
    fn cap26_scheme() {
        assert_eq!(
            DerivationPath::placeholder_cap26().scheme(),
            DerivationPathScheme::Cap26
        );
    }

    #[test]
    fn cap26_hdpath() {
        assert_eq!(
            DerivationPath::placeholder_cap26().hd_path(),
            AccountPath::placeholder().hd_path()
        );
    }

    #[test]
    fn bip44like_scheme() {
        assert_eq!(
            DerivationPath::BIP44Like(BIP44LikePath::new(0)).scheme(),
            DerivationPathScheme::Bip44Olympia
        );
    }

    #[test]
    fn bip44like_hdpath() {
        assert_eq!(
            DerivationPath::BIP44Like(BIP44LikePath::new(0)).hd_path(),
            BIP44LikePath::new(0).hd_path()
        );
    }

    #[test]
    fn into_from_account_bip44_path() {
        assert_eq!(
            DerivationPath::BIP44Like(BIP44LikePath::placeholder()),
            BIP44LikePath::placeholder().into()
        );
    }

    #[test]
    fn into_from_account_cap26_path() {
        assert_eq!(
            DerivationPath::CAP26(AccountPath::placeholder().into()),
            AccountPath::placeholder().into()
        );
    }

    #[test]
    fn into_from_getid_path() {
        assert_eq!(
            DerivationPath::CAP26(GetIDPath::default().into()),
            GetIDPath::default().into()
        );
    }

    #[test]
    fn json_cap26_account() {
        let path = AccountPath::new(NetworkID::Mainnet, CAP26KeyKind::TransactionSigning, 0);
        let model: DerivationPath = path.into();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
        {
			"scheme": "cap26",
			"path": "m/44H/1022H/1H/525H/1460H/0H"
		}
        "#,
        );
    }

    #[test]
    fn json_cap26_getid() {
        let path = GetIDPath::default();
        let model: DerivationPath = path.into();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
        {
			"scheme": "cap26",
			"path": "m/44H/1022H/365H"
		}
        "#,
        );
    }

    #[test]
    fn json_bip44like_account() {
        let path = BIP44LikePath::placeholder();
        let model: DerivationPath = path.into();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
        {
			"scheme": "bip44Olympia",
			"path": "m/44H/1022H/0H/0/0H"
		}
        "#,
        );
    }
}
