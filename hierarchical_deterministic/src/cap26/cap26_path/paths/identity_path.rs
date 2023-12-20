use derive_getters::Getters;
use serde::{de, Deserializer, Serialize, Serializer};
use wallet_kit_common::{error::hdpath_error::HDPathError, network_id::NetworkID};

use crate::{
    bip32::{hd_path::HDPath, hd_path_component::HDPathValue},
    cap26::{
        cap26_entity_kind::CAP26EntityKind, cap26_key_kind::CAP26KeyKind,
        cap26_path::cap26_path::CAP26Path, cap26_repr::CAP26Repr,
    },
    derivation::{
        derivation::Derivation, derivation_path::DerivationPath,
        derivation_path_scheme::DerivationPathScheme,
    },
};

use super::is_entity_path::IsEntityPath;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Getters)]
pub struct IdentityPath {
    path: HDPath,
    network_id: NetworkID,

    #[getter(skip)] // IsEntityPath trait has `entity_kind()` method
    entity_kind: CAP26EntityKind,

    key_kind: CAP26KeyKind,
    index: HDPathValue,
}

impl IsEntityPath for IdentityPath {
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

impl TryFrom<&HDPath> for IdentityPath {
    type Error = HDPathError;

    fn try_from(value: &HDPath) -> Result<Self, Self::Error> {
        Self::try_from_hdpath(value)
    }
}

impl CAP26Repr for IdentityPath {
    fn entity_kind() -> Option<CAP26EntityKind> {
        Some(CAP26EntityKind::Identity)
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
impl IdentityPath {
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder() -> Self {
        Self::from_str("m/44H/1022H/1H/618H/1460H/0H").unwrap()
    }
}

impl Serialize for IdentityPath {
    /// Serializes this `IdentityPath` into its bech32 address string as JSON.
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for IdentityPath {
    /// Tries to deserializes a JSON string as a bech32 address into an `IdentityPath`.
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<IdentityPath, D::Error> {
        let s = String::deserialize(d)?;
        IdentityPath::from_str(&s).map_err(de::Error::custom)
    }
}

impl TryInto<IdentityPath> for &str {
    type Error = HDPathError;

    fn try_into(self) -> Result<IdentityPath, Self::Error> {
        IdentityPath::from_str(self)
    }
}

impl Derivation for IdentityPath {
    fn hd_path(&self) -> &HDPath {
        &self.path
    }

    fn derivation_path(&self) -> DerivationPath {
        DerivationPath::CAP26(CAP26Path::IdentityPath(self.clone()))
    }

    fn scheme(&self) -> DerivationPathScheme {
        DerivationPathScheme::Cap26
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bip32::hd_path::HDPath,
        cap26::{
            cap26_entity_kind::CAP26EntityKind, cap26_key_kind::CAP26KeyKind,
            cap26_path::paths::is_entity_path::IsEntityPath, cap26_repr::CAP26Repr,
        },
        derivation::{derivation::Derivation, derivation_path_scheme::DerivationPathScheme},
    };
    use serde_json::json;
    use wallet_kit_common::{
        error::hdpath_error::HDPathError,
        json::{assert_json_value_eq_after_roundtrip, assert_json_value_ne_after_roundtrip},
        network_id::NetworkID,
    };

    use super::IdentityPath;

    #[test]
    fn entity_kind() {
        assert_eq!(IdentityPath::entity_kind(), Some(CAP26EntityKind::Identity));
    }

    #[test]
    fn hd_path() {
        let str = "m/44H/1022H/1H/618H/1460H/0H";
        let parsed: IdentityPath = str.try_into().unwrap();
        assert_eq!(parsed.hd_path().depth(), 6);
    }

    #[test]
    fn string_roundtrip() {
        let str = "m/44H/1022H/1H/618H/1460H/0H";
        let parsed: IdentityPath = str.try_into().unwrap();
        assert_eq!(parsed.network_id, NetworkID::Mainnet);
        assert_eq!(parsed.entity_kind, CAP26EntityKind::Identity);
        assert_eq!(parsed.key_kind, CAP26KeyKind::TransactionSigning);
        assert_eq!(parsed.index, 0);
        assert_eq!(parsed.to_string(), str);
        let built = IdentityPath::new(NetworkID::Mainnet, CAP26KeyKind::TransactionSigning, 0);
        assert_eq!(built, parsed)
    }

    #[test]
    fn new_tx_sign() {
        assert_eq!(
            IdentityPath::new_mainnet_transaction_signing(77).to_string(),
            "m/44H/1022H/1H/618H/1460H/77H"
        );
    }

    #[test]
    fn invalid_depth() {
        assert_eq!(
            IdentityPath::from_str("m/44H/1022H"),
            Err(HDPathError::InvalidDepthOfCAP26Path)
        )
    }

    #[test]
    fn not_all_hardened() {
        assert_eq!(
            IdentityPath::from_str("m/44H/1022H/1H/618H/1460H/0"), // last not hardened
            Err(HDPathError::NotAllComponentsAreHardened)
        )
    }

    #[test]
    fn cointype_not_found() {
        assert_eq!(
            IdentityPath::from_str("m/44H/33H/1H/618H/1460H/0"), // `33` instead of 1022
            Err(HDPathError::CoinTypeNotFound(33))
        )
    }

    #[test]
    fn fails_when_entity_type_identity() {
        assert_eq!(
            IdentityPath::from_str("m/44H/1022H/1H/525H/1460H/0H"),
            Err(HDPathError::WrongEntityKind(
                CAP26EntityKind::Account.discriminant(),
                CAP26EntityKind::Identity.discriminant()
            ))
        )
    }

    #[test]
    fn fails_when_entity_type_does_not_exist() {
        assert_eq!(
            IdentityPath::from_str("m/44H/1022H/1H/99999H/1460H/0H"),
            Err(HDPathError::InvalidEntityKind(99999))
        )
    }

    #[test]
    fn fails_when_key_kind_does_not_exist() {
        assert_eq!(
            IdentityPath::from_str("m/44H/1022H/1H/618H/22222H/0H"),
            Err(HDPathError::InvalidKeyKind(22222))
        )
    }

    #[test]
    fn fails_when_network_id_is_out_of_bounds() {
        assert_eq!(
            IdentityPath::from_str("m/44H/1022H/4444H/618H/1460H/0H"),
            Err(HDPathError::InvalidNetworkIDExceedsLimit(4444))
        )
    }

    #[test]
    fn fails_when_not_bip44() {
        assert_eq!(
            IdentityPath::from_str("m/777H/1022H/1H/618H/1460H/0H"),
            Err(HDPathError::BIP44PurposeNotFound(777))
        )
    }

    #[test]
    fn missing_leading_m_is_ok() {
        assert!(IdentityPath::from_str("44H/1022H/1H/618H/1460H/0H").is_ok())
    }

    #[test]
    fn fails_when_index_is_too_large() {
        assert_eq!(
            IdentityPath::from_str("m/44H/1022H/1H/618H/1460H/4294967296H"),
            Err(HDPathError::InvalidBIP32Path(
                "m/44H/1022H/1H/618H/1460H/4294967296H".to_string()
            ))
        )
    }

    #[test]
    fn inequality_different_index() {
        let a: IdentityPath = "m/44H/1022H/1H/618H/1460H/0H".try_into().unwrap();
        let b: IdentityPath = "m/44H/1022H/1H/618H/1460H/1H".try_into().unwrap();
        assert!(a != b);
    }
    #[test]
    fn inequality_different_network_id() {
        let a: IdentityPath = "m/44H/1022H/1H/618H/1460H/0H".try_into().unwrap();
        let b: IdentityPath = "m/44H/1022H/2H/618H/1460H/0H".try_into().unwrap();
        assert!(a != b);
    }

    #[test]
    fn inequality_different_key_kind() {
        let a: IdentityPath = "m/44H/1022H/1H/618H/1460H/0H".try_into().unwrap();
        let b: IdentityPath = "m/44H/1022H/1H/618H/1678H/0H".try_into().unwrap();
        assert!(a != b);
    }

    #[test]
    fn json_roundtrip() {
        let str = "m/44H/1022H/1H/618H/1460H/0H";
        let parsed: IdentityPath = str.try_into().unwrap();
        assert_json_value_eq_after_roundtrip(&parsed, json!(str));
        assert_json_value_ne_after_roundtrip(&parsed, json!("m/44H/1022H/1H/618H/1460H/1H"));
    }

    #[test]
    fn identity_path_scheme() {
        assert_eq!(
            IdentityPath::placeholder().scheme(),
            DerivationPathScheme::Cap26
        );
    }

    #[test]
    fn identity_path_derivation_path() {
        assert_eq!(
            IdentityPath::placeholder()
                .derivation_path()
                .hd_path()
                .to_string(),
            "m/44H/1022H/1H/618H/1460H/0H"
        );
    }

    #[test]
    fn try_from_hdpath_valid() {
        let hdpath = HDPath::from_str("m/44H/1022H/1H/618H/1460H/0H").unwrap();
        assert!(IdentityPath::try_from(&hdpath).is_ok());
    }

    #[test]
    fn try_from_hdpath_invalid() {
        let hdpath = HDPath::from_str("m/44H/1022H/1H/525H/1460H/0H").unwrap();
        assert_eq!(
            IdentityPath::try_from(&hdpath),
            Err(HDPathError::WrongEntityKind(525, 618))
        );
    }

    #[test]
    fn is_entity_path_index() {
        let sut = IdentityPath::placeholder();
        assert_eq!(sut.index(), &0);
        assert_eq!(sut.network_id(), &NetworkID::Mainnet);
        assert_eq!(sut.key_kind(), &CAP26KeyKind::TransactionSigning);
    }
}
