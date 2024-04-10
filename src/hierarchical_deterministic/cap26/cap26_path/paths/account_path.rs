use crate::prelude::*;

#[derive(
    Zeroize,
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
    uniffi::Record,
)]
#[display("{}", self.bip32_string())]
pub struct AccountPath {
    pub path: HDPath,

    #[zeroize(skip)]
    pub network_id: NetworkID,

    #[zeroize(skip)]
    pub entity_kind: CAP26EntityKind,

    #[zeroize(skip)]
    pub key_kind: CAP26KeyKind,

    pub index: HDPathValue,
}

impl IsNetworkAware for AccountPath {
    fn network_id(&self) -> NetworkID {
        self.network_id
    }
}

impl IsEntityPath for AccountPath {
    fn key_kind(&self) -> CAP26KeyKind {
        self.key_kind
    }

    fn index(&self) -> HDPathValue {
        self.index
    }
}

impl TryFrom<CAP26Path> for AccountPath {
    type Error = CommonError;

    fn try_from(value: CAP26Path) -> Result<Self, Self::Error> {
        value
            .as_account()
            .ok_or(CommonError::ExpectedAccountPathButGotSomethingElse)
            .cloned()
    }
}

impl TryFrom<&HDPath> for AccountPath {
    type Error = CommonError;

    fn try_from(value: &HDPath) -> Result<Self> {
        Self::try_from_hdpath(value)
    }
}

impl EntityCAP26Path for AccountPath {
    fn entity_kind() -> CAP26EntityKind {
        CAP26EntityKind::Account
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

impl HasSampleValues for AccountPath {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::from_str("m/44H/1022H/1H/525H/1460H/0H").unwrap()
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::from_str("m/44H/1022H/1H/525H/1460H/1H").unwrap()
    }
}

impl FromStr for AccountPath {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bip32str(s)
    }
}

impl Derivation for AccountPath {
    fn hd_path(&self) -> &HDPath {
        &self.path
    }
    fn derivation_path(&self) -> DerivationPath {
        DerivationPath::CAP26 {
            value: CAP26Path::Account {
                value: self.clone(),
            },
        }
    }
    fn scheme(&self) -> DerivationPathScheme {
        DerivationPathScheme::Cap26
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(AccountPath::sample(), AccountPath::sample());
        assert_eq!(AccountPath::sample_other(), AccountPath::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(AccountPath::sample(), AccountPath::sample_other());
    }

    #[test]
    fn index() {
        assert_eq!(AccountPath::sample().index(), 0);
    }

    #[test]
    fn entity_kind() {
        assert_eq!(AccountPath::sample().entity_kind, CAP26EntityKind::Account);
    }

    #[test]
    fn hd_path() {
        let str = "m/44H/1022H/1H/525H/1460H/0H";
        let parsed: AccountPath = str.parse().unwrap();
        assert_eq!(parsed.hd_path().depth(), 6);
    }

    #[test]
    fn string_roundtrip() {
        let str = "m/44H/1022H/1H/525H/1460H/0H";
        let parsed: AccountPath = str.parse().unwrap();
        assert_eq!(parsed.network_id, NetworkID::Mainnet);
        assert_eq!(parsed.entity_kind, CAP26EntityKind::Account);
        assert_eq!(parsed.key_kind, CAP26KeyKind::TransactionSigning);
        assert_eq!(parsed.index, 0);
        assert_eq!(parsed.to_string(), str);
        let built = AccountPath::new(
            NetworkID::Mainnet,
            CAP26KeyKind::TransactionSigning,
            0,
        );
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
            Err(CommonError::InvalidDepthOfCAP26Path {
                expected: 6,
                found: 2
            })
        )
    }

    #[test]
    fn not_all_hardened() {
        assert_eq!(
            AccountPath::from_str("m/44H/1022H/1H/525H/1460H/0"), // last not hardened
            Err(CommonError::NotAllComponentsAreHardened)
        )
    }

    #[test]
    fn cointype_not_found() {
        assert_eq!(
            AccountPath::from_str("m/44H/33H/1H/525H/1460H/0"), // `33` instead of 1022
            Err(CommonError::CoinTypeNotFound { bad_value: 33 })
        )
    }

    #[test]
    fn fails_when_entity_type_identity() {
        assert_eq!(
            AccountPath::from_str("m/44H/1022H/1H/618H/1460H/0H"),
            Err(CommonError::WrongEntityKind {
                expected: CAP26EntityKind::Account,
                found: CAP26EntityKind::Identity
            })
        )
    }

    #[test]
    fn fails_when_entity_type_does_not_exist() {
        assert_eq!(
            AccountPath::from_str("m/44H/1022H/1H/99999H/1460H/0H"),
            Err(CommonError::InvalidEntityKind { bad_value: 99999 })
        )
    }

    #[test]
    fn fails_when_key_kind_does_not_exist() {
        assert_eq!(
            AccountPath::from_str("m/44H/1022H/1H/525H/22222H/0H"),
            Err(CommonError::InvalidKeyKind { bad_value: 22222 })
        )
    }

    #[test]
    fn fails_when_network_id_is_out_of_bounds() {
        assert_eq!(
            AccountPath::from_str("m/44H/1022H/4444H/525H/1460H/0H"),
            Err(CommonError::InvalidNetworkIDExceedsLimit { bad_value: 4444 })
        )
    }

    #[test]
    fn fails_when_not_bip44() {
        assert_eq!(
            AccountPath::from_str("m/777H/1022H/1H/525H/1460H/0H"),
            Err(CommonError::BIP44PurposeNotFound { bad_value: 777 })
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
            Err(CommonError::InvalidBIP32Path {
                bad_value: "m/44H/1022H/1H/525H/1460H/4294967296H".to_string()
            })
        )
    }

    #[test]
    fn inequality_different_index() {
        let a: AccountPath = "m/44H/1022H/1H/525H/1460H/0H".parse().unwrap();
        let b: AccountPath = "m/44H/1022H/1H/525H/1460H/1H".parse().unwrap();
        assert!(a != b);
    }
    #[test]
    fn inequality_different_network_id() {
        let a: AccountPath = "m/44H/1022H/1H/525H/1460H/0H".parse().unwrap();
        let b: AccountPath = "m/44H/1022H/2H/525H/1460H/0H".parse().unwrap();
        assert!(a != b);
    }

    #[test]
    fn inequality_different_key_kind() {
        let a: AccountPath = "m/44H/1022H/1H/525H/1460H/0H".parse().unwrap();
        let b: AccountPath = "m/44H/1022H/1H/525H/1678H/0H".parse().unwrap();
        assert!(a != b);
    }

    #[test]
    fn json_roundtrip() {
        let str = "m/44H/1022H/1H/525H/1460H/0H";
        let parsed: AccountPath = str.parse().unwrap();
        assert_json_value_eq_after_roundtrip(&parsed, json!(str));
        assert_json_value_ne_after_roundtrip(
            &parsed,
            json!("m/44H/1022H/1H/525H/1460H/1H"),
        );
    }

    #[test]
    fn is_entity_path_index() {
        let sut = AccountPath::sample();
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
            Err(CommonError::WrongEntityKind {
                expected: CAP26EntityKind::Account,
                found: CAP26EntityKind::Identity
            })
        );
    }
}
