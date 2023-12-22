use crate::HDPathError;
use derive_getters::Getters;
use serde::{de, Deserializer, Serialize, Serializer};

#[cfg(any(test, feature = "placeholder"))]
use crate::HasPlaceholder;

use crate::{
    CAP26EntityKind, CAP26KeyKind, CAP26Path, CAP26Repr, Derivation, DerivationPath,
    DerivationPathScheme, HDPath, HDPathValue, NetworkID,
};

use super::is_entity_path::IsEntityPath;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Getters)]
pub struct AccountPath {
    path: HDPath,

    #[getter(skip)] // IsEntityPath trait has `network_id()` method
    network_id: NetworkID,

    entity_kind: CAP26EntityKind,

    #[getter(skip)] // IsEntityPath trait has `key_kind()` method
    key_kind: CAP26KeyKind,

    #[getter(skip)] // IsEntityPath trait has `index()` method
    index: HDPathValue,
}

impl IsEntityPath for AccountPath {
    fn network_id(&self) -> NetworkID {
        self.network_id
    }

    fn key_kind(&self) -> CAP26KeyKind {
        self.key_kind
    }

    fn index(&self) -> HDPathValue {
        self.index
    }
}

impl TryFrom<&HDPath> for AccountPath {
    type Error = HDPathError;

    fn try_from(value: &HDPath) -> Result<Self, Self::Error> {
        Self::try_from_hdpath(value)
    }
}

impl CAP26Repr for AccountPath {
    fn entity_kind() -> Option<CAP26EntityKind> {
        Some(CAP26EntityKind::Account)
    }

    fn __with_path_and_components(
        path: HDPath,
        network_id: NetworkID,
        entity_kind: CAP26EntityKind,
        key_kind: CAP26KeyKind,
        index: HDPathValue,
    ) -> Self {
        Self {
            path,
            network_id,
            entity_kind,
            key_kind,
            index,
        }
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl HasPlaceholder for AccountPath {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::from_str("m/44H/1022H/1H/525H/1460H/0H").unwrap()
    }

    /// A placeholder used to facilitate unit tests.
    fn placeholder_other() -> Self {
        Self::from_str("m/44H/1022H/1H/525H/1460H/1H").unwrap()
    }
}

impl Serialize for AccountPath {
    /// Serializes this `AccountPath` into JSON as a string on: "m/44H/1022H/1H/525H/1460H/0H" format
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for AccountPath {
    /// Tries to deserializes a JSON string as a derivation path string into a `AccountPath`
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<AccountPath, D::Error> {
        let s = String::deserialize(d)?;
        AccountPath::from_str(&s).map_err(de::Error::custom)
    }
}

impl TryInto<AccountPath> for &str {
    type Error = HDPathError;

    fn try_into(self) -> Result<AccountPath, Self::Error> {
        AccountPath::from_str(self)
    }
}

impl Derivation for AccountPath {
    fn hd_path(&self) -> &HDPath {
        &self.path
    }
    fn derivation_path(&self) -> DerivationPath {
        DerivationPath::CAP26(CAP26Path::AccountPath(self.clone()))
    }
    fn scheme(&self) -> DerivationPathScheme {
        DerivationPathScheme::Cap26
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        HDPathError, HasPlaceholder,
        {assert_json_value_eq_after_roundtrip, assert_json_value_ne_after_roundtrip},
    };
    use serde_json::json;

    use crate::{
        CAP26EntityKind, CAP26KeyKind, CAP26Repr, Derivation, HDPath, IsEntityPath, NetworkID,
    };

    use super::AccountPath;

    #[test]
    fn equality() {
        assert_eq!(AccountPath::placeholder(), AccountPath::placeholder());
        assert_eq!(
            AccountPath::placeholder_other(),
            AccountPath::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(AccountPath::placeholder(), AccountPath::placeholder_other());
    }

    #[test]
    fn index() {
        assert_eq!(AccountPath::placeholder().index(), 0);
    }

    #[test]
    fn entity_kind() {
        assert_eq!(
            AccountPath::placeholder().entity_kind(),
            &CAP26EntityKind::Account
        );
    }

    #[test]
    fn hd_path() {
        let str = "m/44H/1022H/1H/525H/1460H/0H";
        let parsed: AccountPath = str.try_into().unwrap();
        assert_eq!(parsed.hd_path().depth(), 6);
    }

    #[test]
    fn string_roundtrip() {
        let str = "m/44H/1022H/1H/525H/1460H/0H";
        let parsed: AccountPath = str.try_into().unwrap();
        assert_eq!(parsed.network_id, NetworkID::Mainnet);
        assert_eq!(parsed.entity_kind, CAP26EntityKind::Account);
        assert_eq!(parsed.key_kind, CAP26KeyKind::TransactionSigning);
        assert_eq!(parsed.index, 0);
        assert_eq!(parsed.to_string(), str);
        let built = AccountPath::new(NetworkID::Mainnet, CAP26KeyKind::TransactionSigning, 0);
        assert_eq!(built, parsed)
    }

    #[test]
    fn new_tx_sign() {
        assert_eq!(
            AccountPath::new_mainnet_transaction_signing(77).to_string(),
            "m/44H/1022H/1H/525H/1460H/77H"
        );
    }

    #[test]
    fn invalid_depth() {
        assert_eq!(
            AccountPath::from_str("m/44H/1022H"),
            Err(HDPathError::InvalidDepthOfCAP26Path)
        )
    }

    #[test]
    fn not_all_hardened() {
        assert_eq!(
            AccountPath::from_str("m/44H/1022H/1H/525H/1460H/0"), // last not hardened
            Err(HDPathError::NotAllComponentsAreHardened)
        )
    }

    #[test]
    fn cointype_not_found() {
        assert_eq!(
            AccountPath::from_str("m/44H/33H/1H/525H/1460H/0"), // `33` instead of 1022
            Err(HDPathError::CoinTypeNotFound(33))
        )
    }

    #[test]
    fn fails_when_entity_type_identity() {
        assert_eq!(
            AccountPath::from_str("m/44H/1022H/1H/618H/1460H/0H"),
            Err(HDPathError::WrongEntityKind(
                CAP26EntityKind::Identity.discriminant(),
                CAP26EntityKind::Account.discriminant()
            ))
        )
    }

    #[test]
    fn fails_when_entity_type_does_not_exist() {
        assert_eq!(
            AccountPath::from_str("m/44H/1022H/1H/99999H/1460H/0H"),
            Err(HDPathError::InvalidEntityKind(99999))
        )
    }

    #[test]
    fn fails_when_key_kind_does_not_exist() {
        assert_eq!(
            AccountPath::from_str("m/44H/1022H/1H/525H/22222H/0H"),
            Err(HDPathError::InvalidKeyKind(22222))
        )
    }

    #[test]
    fn fails_when_network_id_is_out_of_bounds() {
        assert_eq!(
            AccountPath::from_str("m/44H/1022H/4444H/525H/1460H/0H"),
            Err(HDPathError::InvalidNetworkIDExceedsLimit(4444))
        )
    }

    #[test]
    fn fails_when_not_bip44() {
        assert_eq!(
            AccountPath::from_str("m/777H/1022H/1H/525H/1460H/0H"),
            Err(HDPathError::BIP44PurposeNotFound(777))
        )
    }

    #[test]
    fn missing_leading_m_is_ok() {
        assert!(AccountPath::from_str("44H/1022H/1H/525H/1460H/0H").is_ok())
    }

    #[test]
    fn fails_when_index_is_too_large() {
        assert_eq!(
            AccountPath::from_str("m/44H/1022H/1H/525H/1460H/4294967296H"),
            Err(HDPathError::InvalidBIP32Path(
                "m/44H/1022H/1H/525H/1460H/4294967296H".to_string()
            ))
        )
    }

    #[test]
    fn inequality_different_index() {
        let a: AccountPath = "m/44H/1022H/1H/525H/1460H/0H".try_into().unwrap();
        let b: AccountPath = "m/44H/1022H/1H/525H/1460H/1H".try_into().unwrap();
        assert!(a != b);
    }
    #[test]
    fn inequality_different_network_id() {
        let a: AccountPath = "m/44H/1022H/1H/525H/1460H/0H".try_into().unwrap();
        let b: AccountPath = "m/44H/1022H/2H/525H/1460H/0H".try_into().unwrap();
        assert!(a != b);
    }

    #[test]
    fn inequality_different_key_kind() {
        let a: AccountPath = "m/44H/1022H/1H/525H/1460H/0H".try_into().unwrap();
        let b: AccountPath = "m/44H/1022H/1H/525H/1678H/0H".try_into().unwrap();
        assert!(a != b);
    }

    #[test]
    fn json_roundtrip() {
        let str = "m/44H/1022H/1H/525H/1460H/0H";
        let parsed: AccountPath = str.try_into().unwrap();
        assert_json_value_eq_after_roundtrip(&parsed, json!(str));
        assert_json_value_ne_after_roundtrip(&parsed, json!("m/44H/1022H/1H/525H/1460H/1H"));
    }

    #[test]
    fn is_entity_path_index() {
        let sut = AccountPath::placeholder();
        assert_eq!(sut.index(), 0);
        assert_eq!(sut.network_id(), NetworkID::Mainnet);
        assert_eq!(sut.key_kind(), CAP26KeyKind::TransactionSigning);
    }

    #[test]
    fn try_from_hdpath_valid() {
        let hdpath = HDPath::from_str("m/44H/1022H/1H/525H/1460H/0H").unwrap();
        let account_path = AccountPath::try_from(&hdpath);
        assert!(account_path.is_ok());
    }

    #[test]
    fn try_from_hdpath_invalid() {
        let hdpath = HDPath::from_str("m/44H/1022H/1H/618H/1460H/0H").unwrap();
        assert_eq!(
            AccountPath::try_from(&hdpath),
            Err(HDPathError::WrongEntityKind(618, 525))
        );
    }
}
