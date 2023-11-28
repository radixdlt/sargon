use serde::{de, Deserializer, Serialize, Serializer};
use wallet_kit_common::network_id::NetworkID;

use crate::{
    bip32::{hd_path::HDPath, hd_path_component::HDPathValue},
    cap26::{
        cap26_entity_kind::CAP26EntityKind, cap26_key_kind::CAP26KeyKind, cap26_repr::CAP26Repr,
    },
    derivation::{derivation::Derivation, derivation_path_scheme::DerivationPathScheme},
    hdpath_error::HDPathError,
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountPath {
    pub path: HDPath,
    pub network_id: NetworkID,
    pub entity_kind: CAP26EntityKind,
    pub key_kind: CAP26KeyKind,
    pub index: HDPathValue,
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

impl AccountPath {
    pub fn placeholder() -> Self {
        Self::from_str("m/44H/1022H/1H/525H/1460H/0H").unwrap()
    }
}

impl Serialize for AccountPath {
    /// Serializes this `AccountAddress` into its bech32 address string as JSON.
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for AccountPath {
    /// Tries to deserializes a JSON string as a bech32 address into an `AccountAddress`.
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

    fn scheme(&self) -> DerivationPathScheme {
        DerivationPathScheme::Cap26
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use wallet_kit_common::{
        json::{assert_json_value_eq_after_roundtrip, assert_json_value_ne_after_roundtrip},
        network_id::NetworkID,
    };

    use crate::{
        cap26::{
            cap26_entity_kind::CAP26EntityKind, cap26_key_kind::CAP26KeyKind, cap26_repr::CAP26Repr,
        },
        derivation::derivation::Derivation,
        hdpath_error::HDPathError,
    };

    use super::AccountPath;

    #[test]
    fn entity_kind() {
        assert_eq!(AccountPath::entity_kind(), Some(CAP26EntityKind::Account));
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
                CAP26EntityKind::Identity,
                CAP26EntityKind::Account
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
}
