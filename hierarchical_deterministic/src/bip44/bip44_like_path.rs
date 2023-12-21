use serde::{de, Deserializer, Serialize, Serializer};
use wallet_kit_common::error::hdpath_error::HDPathError;

use crate::{
    bip32::{HDPath, HDPathComponent, HDPathValue},
    Derivation, DerivationPath, DerivationPathScheme,
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BIP44LikePath(HDPath);

impl TryFrom<&HDPath> for BIP44LikePath {
    type Error = HDPathError;

    fn try_from(value: &HDPath) -> Result<Self, Self::Error> {
        let (path, components) =
            HDPath::try_parse_base_hdpath(value, HDPathError::InvalidDepthOfBIP44Path)?;
        if path.depth() != 5 {
            return Err(HDPathError::InvalidDepthOfBIP44Path);
        }
        let account = &components[2];
        if !account.is_hardened() {
            return Err(HDPathError::InvalidBIP44LikePathAccountWasNotHardened);
        }
        let change = &components[3];
        if change.is_hardened() {
            return Err(HDPathError::InvalidBIP44LikePathChangeWasUnexpectedlyHardened);
        }

        let index = &components[4];
        if !index.is_hardened() {
            return Err(HDPathError::InvalidBIP44LikePathIndexWasNotHardened);
        }
        return Ok(Self(path));
    }
}

impl BIP44LikePath {
    pub fn from_str(s: &str) -> Result<Self, HDPathError> {
        let (path, _) = HDPath::try_parse_base(s, HDPathError::InvalidDepthOfBIP44Path)?;
        return Self::try_from(&path);
    }

    fn with_account_and_index(account: HDPathValue, index: HDPathValue) -> Self {
        let c0 = HDPathComponent::bip44_purpose(); // purpose
        let c1 = HDPathComponent::bip44_cointype(); // cointype
        let c2 = HDPathComponent::harden(account); // account
        let c3 = HDPathComponent::from_value(0); // change
        let c4 = HDPathComponent::harden(index); // index
        let components = vec![c0, c1, c2, c3, c4];
        let path = HDPath::from_components(components);
        return Self(path);
    }

    pub fn new(index: HDPathValue) -> Self {
        Self::with_account_and_index(0, index)
    }
}

impl Derivation for BIP44LikePath {
    fn derivation_path(&self) -> DerivationPath {
        DerivationPath::BIP44Like(self.clone())
    }
    fn hd_path(&self) -> &HDPath {
        &self.0
    }

    fn scheme(&self) -> DerivationPathScheme {
        DerivationPathScheme::Bip44Olympia
    }
}

impl Serialize for BIP44LikePath {
    /// Serializes this `BIP44LikePath` into JSON as a string on: "m/44H/1022H/0H/0/0H" format
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for BIP44LikePath {
    /// Tries to deserializes a JSON string as a derivation path string into a `BIP44LikePath`
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<BIP44LikePath, D::Error> {
        let s = String::deserialize(d)?;
        BIP44LikePath::from_str(&s).map_err(de::Error::custom)
    }
}

impl TryInto<BIP44LikePath> for &str {
    type Error = HDPathError;

    fn try_into(self) -> Result<BIP44LikePath, Self::Error> {
        BIP44LikePath::from_str(self)
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl BIP44LikePath {
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder() -> Self {
        Self::from_str("m/44H/1022H/0H/0/0H").expect("Valid placeholder")
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use wallet_kit_common::{
        error::hdpath_error::HDPathError,
        json::{assert_json_value_eq_after_roundtrip, assert_json_value_ne_after_roundtrip},
    };

    use crate::Derivation;

    use super::BIP44LikePath;

    #[test]
    fn string_roundtrip() {
        let str = "m/44H/1022H/0H/0/0H";
        let a: BIP44LikePath = str.try_into().unwrap();
        assert_eq!(a.to_string(), str);
    }

    #[test]
    fn placeholder() {
        assert_eq!(
            BIP44LikePath::placeholder().to_string(),
            "m/44H/1022H/0H/0/0H"
        );
    }

    #[test]
    fn invalid_depth_1() {
        assert_eq!(
            BIP44LikePath::from_str("m/44H"),
            Err(HDPathError::InvalidDepthOfBIP44Path)
        );
    }

    #[test]
    fn invalid_depth() {
        assert_eq!(
            BIP44LikePath::from_str("m/44H/1022H/0H"),
            Err(HDPathError::InvalidDepthOfBIP44Path)
        );
    }

    #[test]
    fn invalid_account_not_hardened() {
        assert_eq!(
            BIP44LikePath::from_str("m/44H/1022H/0/1/2H"),
            Err(HDPathError::InvalidBIP44LikePathAccountWasNotHardened)
        );
    }

    #[test]
    fn invalid_change_was_hardened() {
        assert_eq!(
            BIP44LikePath::from_str("m/44H/1022H/0H/0H/2H"),
            Err(HDPathError::InvalidBIP44LikePathChangeWasUnexpectedlyHardened)
        );
    }

    #[test]
    fn invalid_index_not_hardened() {
        assert_eq!(
            BIP44LikePath::from_str("m/44H/1022H/0H/0/0"),
            Err(HDPathError::InvalidBIP44LikePathIndexWasNotHardened)
        );
    }

    #[test]
    fn inequality_different_accounts() {
        let a: BIP44LikePath = "m/44H/1022H/0H/0/0H".try_into().unwrap();
        let b: BIP44LikePath = "m/44H/1022H/1H/0/0H".try_into().unwrap();
        assert!(a != b);
    }

    #[test]
    fn inequality_different_index() {
        let a: BIP44LikePath = "m/44H/1022H/0H/0/0H".try_into().unwrap();
        let b: BIP44LikePath = "m/44H/1022H/0H/0/1H".try_into().unwrap();
        assert!(a != b);
    }

    #[test]
    fn json_roundtrip() {
        let str = "m/44H/1022H/0H/0/0H";
        let parsed: BIP44LikePath = str.try_into().unwrap();
        assert_json_value_eq_after_roundtrip(&parsed, json!(str));
        assert_json_value_ne_after_roundtrip(&parsed, json!("m/44H/1022H/0H/0/1H"));
    }

    #[test]
    fn new_with_account() {
        assert_ne!(
            BIP44LikePath::with_account_and_index(1, 0),
            BIP44LikePath::new(0)
        );
    }
}
