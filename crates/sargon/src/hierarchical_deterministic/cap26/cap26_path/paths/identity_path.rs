use crate::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    SerializeDisplay,
    DeserializeFromStr,
    derive_more::Display,
)]
#[display("{}", self.bip32_string())]
pub struct IdentityPath {
    pub path: HDPath,

    pub network_id: NetworkID,

    pub entity_kind: CAP26EntityKind,

    pub key_kind: CAP26KeyKind,

    pub index: HDPathValue,
}

impl IsNetworkAware for IdentityPath {
    fn network_id(&self) -> NetworkID {
        self.network_id
    }
}

impl IsEntityPath for IdentityPath {
    fn key_kind(&self) -> CAP26KeyKind {
        self.key_kind
    }

    fn index(&self) -> HDPathValue {
        self.index
    }
}

impl TryFrom<CAP26Path> for IdentityPath {
    type Error = CommonError;

    fn try_from(value: CAP26Path) -> Result<Self, Self::Error> {
        value
            .as_identity()
            .ok_or(CommonError::ExpectedIdentityPathButGotSomethingElse)
            .cloned()
    }
}

impl TryFrom<&HDPath> for IdentityPath {
    type Error = CommonError;

    fn try_from(value: &HDPath) -> Result<Self> {
        Self::try_from_hdpath(value)
    }
}

impl EntityCAP26Path for IdentityPath {
    fn entity_kind() -> CAP26EntityKind {
        CAP26EntityKind::Identity
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

impl HasSampleValues for IdentityPath {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::from_str("m/44H/1022H/1H/618H/1460H/0H").unwrap()
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::from_str("m/44H/1022H/1H/618H/1460H/1H").unwrap()
    }
}

impl FromStr for IdentityPath {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bip32str(s)
    }
}
impl Derivation for IdentityPath {
    fn hd_path(&self) -> &HDPath {
        &self.path
    }

    fn derivation_path(&self) -> DerivationPath {
        DerivationPath::CAP26 {
            value: CAP26Path::Identity {
                value: self.clone(),
            },
        }
    }

    fn curve(&self) -> SLIP10Curve {
        self.scheme().curve()
    }
}

impl IdentityPath {
    pub fn scheme(&self) -> DerivationPathScheme {
        DerivationPathScheme::Cap26
    }
}

#[cfg(test)]
mod tests {

    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(IdentityPath::sample(), IdentityPath::sample());
        assert_eq!(IdentityPath::sample_other(), IdentityPath::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(IdentityPath::sample(), IdentityPath::sample_other());
    }

    #[test]
    fn index() {
        assert_eq!(IdentityPath::sample().index(), 0);
    }

    #[test]
    fn network_id() {
        assert_eq!(IdentityPath::sample().network_id(), NetworkID::Mainnet);
    }

    #[test]
    fn entity_kind() {
        assert_eq!(
            IdentityPath::sample().entity_kind,
            CAP26EntityKind::Identity
        );
    }

    #[test]
    fn hd_path() {
        let str = "m/44H/1022H/1H/618H/1460H/0H";
        let parsed: IdentityPath = str.parse().unwrap();
        assert_eq!(parsed.hd_path().depth(), 6);
    }

    #[test]
    fn string_roundtrip() {
        let str = "m/44H/1022H/1H/618H/1460H/0H";
        let parsed: IdentityPath = str.parse().unwrap();
        assert_eq!(parsed.network_id, NetworkID::Mainnet);
        assert_eq!(parsed.entity_kind, CAP26EntityKind::Identity);
        assert_eq!(parsed.key_kind, CAP26KeyKind::TransactionSigning);
        assert_eq!(parsed.index, 0);
        assert_eq!(parsed.to_string(), str);
        let built = IdentityPath::new(
            NetworkID::Mainnet,
            CAP26KeyKind::TransactionSigning,
            0,
        );
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
            Err(CommonError::InvalidDepthOfCAP26Path {
                expected: 6,
                found: 2
            })
        )
    }

    #[test]
    fn not_all_hardened() {
        assert_eq!(
            IdentityPath::from_str("m/44H/1022H/1H/618H/1460H/0"), // last not hardened
            Err(CommonError::NotAllComponentsAreHardened)
        )
    }

    #[test]
    fn cointype_not_found() {
        assert_eq!(
            IdentityPath::from_str("m/44H/33H/1H/618H/1460H/0"), // `33` instead of 1022
            Err(CommonError::CoinTypeNotFound { bad_value: 33 })
        )
    }

    #[test]
    fn fails_when_entity_type_identity() {
        assert_eq!(
            IdentityPath::from_str("m/44H/1022H/1H/525H/1460H/0H"),
            Err(CommonError::WrongEntityKind {
                expected: CAP26EntityKind::Identity,
                found: CAP26EntityKind::Account,
            })
        )
    }

    #[test]
    fn fails_when_entity_type_does_not_exist() {
        assert_eq!(
            IdentityPath::from_str("m/44H/1022H/1H/99999H/1460H/0H"),
            Err(CommonError::InvalidEntityKind { bad_value: 99999 })
        )
    }

    #[test]
    fn fails_when_key_kind_does_not_exist() {
        assert_eq!(
            IdentityPath::from_str("m/44H/1022H/1H/618H/22222H/0H"),
            Err(CommonError::InvalidKeyKind { bad_value: 22222 })
        )
    }

    #[test]
    fn fails_when_network_id_is_out_of_bounds() {
        assert_eq!(
            IdentityPath::from_str("m/44H/1022H/4444H/618H/1460H/0H"),
            Err(CommonError::InvalidNetworkIDExceedsLimit { bad_value: 4444 })
        )
    }

    #[test]
    fn fails_when_not_bip44() {
        assert_eq!(
            IdentityPath::from_str("m/777H/1022H/1H/618H/1460H/0H"),
            Err(CommonError::BIP44PurposeNotFound { bad_value: 777 })
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
            Err(CommonError::InvalidBIP32Path {
                bad_value: "m/44H/1022H/1H/618H/1460H/4294967296H".to_string()
            })
        )
    }

    #[test]
    fn inequality_different_index() {
        let a: IdentityPath = "m/44H/1022H/1H/618H/1460H/0H".parse().unwrap();
        let b: IdentityPath = "m/44H/1022H/1H/618H/1460H/1H".parse().unwrap();
        assert!(a != b);
    }
    #[test]
    fn inequality_different_network_id() {
        let a: IdentityPath = "m/44H/1022H/1H/618H/1460H/0H".parse().unwrap();
        let b: IdentityPath = "m/44H/1022H/2H/618H/1460H/0H".parse().unwrap();
        assert!(a != b);
    }

    #[test]
    fn inequality_different_key_kind() {
        let a: IdentityPath = "m/44H/1022H/1H/618H/1460H/0H".parse().unwrap();
        let b: IdentityPath = "m/44H/1022H/1H/618H/1678H/0H".parse().unwrap();
        assert!(a != b);
    }

    #[test]
    fn json_roundtrip() {
        let str = "m/44H/1022H/1H/618H/1460H/0H";
        let parsed: IdentityPath = str.parse().unwrap();
        assert_json_value_eq_after_roundtrip(&parsed, json!(str));
        assert_json_value_ne_after_roundtrip(
            &parsed,
            json!("m/44H/1022H/1H/618H/1460H/1H"),
        );
    }

    #[test]
    fn identity_path_scheme() {
        assert_eq!(
            IdentityPath::sample().scheme(),
            DerivationPathScheme::Cap26
        );
    }

    #[test]
    fn identity_path_derivation_path() {
        assert_eq!(
            IdentityPath::sample()
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
            Err(CommonError::WrongEntityKind {
                expected: CAP26EntityKind::Identity,
                found: CAP26EntityKind::Account
            })
        );
    }

    #[test]
    fn is_entity_path_index() {
        let sut = IdentityPath::sample();
        assert_eq!(sut.index(), 0);
        assert_eq!(sut.network_id(), NetworkID::Mainnet);
        assert_eq!(sut.key_kind(), CAP26KeyKind::TransactionSigning);
    }
}
