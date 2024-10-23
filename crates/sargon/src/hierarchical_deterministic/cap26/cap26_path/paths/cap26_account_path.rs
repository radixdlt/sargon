use crate::prelude::*;

#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    derive_more::Debug,
    SerializeDisplay,
    DeserializeFromStr,
    derive_more::Display,
)]
#[display("{}", self.to_bip32_string())]
#[debug("{}", self.to_bip32_string_debug())]
pub struct AccountPath {
    pub network_id: NetworkID,
    pub key_kind: CAP26KeyKind,
    pub index: Hardened,
}

impl HasKeyKind for AccountPath {
    fn key_kind(&self) -> CAP26KeyKind {
        self.key_kind
    }
}

impl IsNetworkAware for AccountPath {
    fn network_id(&self) -> NetworkID {
        self.network_id
    }
}

impl IsSecurityStateAware for AccountPath {
    fn is_securified(&self) -> bool {
        self.index.is_securified()
    }
}

impl NewEntityPath for AccountPath {
    fn new(
        network_id: impl Into<NetworkID>,
        key_kind: impl Into<CAP26KeyKind>,
        index: impl Into<Hardened>,
    ) -> Self {
        Self {
            network_id: network_id.into(),
            key_kind: key_kind.into(),
            index: index.into(),
        }
    }
}

impl TryFrom<HDPath> for AccountPath {
    type Error = CommonError;
    fn try_from(path: HDPath) -> Result<Self> {
        UnvalidatedCAP26Path::try_from(path)
            .and_then(Self::try_from_unvalidated)
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

impl From<AccountPath> for HDPath {
    fn from(account_path: AccountPath) -> Self {
        account_path.to_hd_path()
    }
}

impl AccountPath {
    pub fn to_hd_path(&self) -> HDPath {
        cap26(
            self.network_id,
            Self::entity_kind(),
            self.key_kind,
            self.index,
        )
    }
}

impl HasEntityKind for AccountPath {
    fn entity_kind() -> CAP26EntityKind {
        CAP26EntityKind::Account
    }
}

impl ToBIP32Str for AccountPath {
    fn to_bip32_string(&self) -> String {
        self.to_hd_path().to_bip32_string()
    }
    fn to_bip32_string_debug(&self) -> String {
        self.to_hd_path().to_bip32_string_debug()
    }
}

impl FromBIP32Str for AccountPath {
    fn from_bip32_string(s: impl AsRef<str>) -> Result<Self> {
        HDPath::from_bip32_string(s).and_then(Self::try_from)
    }
}
impl FromStr for AccountPath {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_bip32_string(s)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    type Sut = AccountPath;

    #[test]
    fn equality() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Sut::sample(), Sut::sample_other());
    }

    #[test]
    fn display() {
        let sut = Sut::sample();
        assert_eq!(format!("{}", sut), "m/44H/1022H/1H/525H/1460H/0H");
    }

    #[test]
    fn debug() {
        let sut = Sut::sample();
        assert_eq!(format!("{:?}", sut), "m/44'/1022'/1'/525'/1460'/0'");
    }

    #[test]
    fn to_bip32_path() {
        let sut = Sut::sample();
        assert_eq!(format!("{}", sut), "m/44H/1022H/1H/525H/1460H/0H");
    }

    #[test]
    fn from_str() {
        let sut = Sut::from_str("m/44H/1022H/1H/525H/1460H/0H").unwrap();
        assert_eq!(sut, Sut::sample());
    }

    #[test]
    fn from_str_securified() {
        let sut = Sut::from_str("m/44H/1022H/1H/525H/1460H/0S").unwrap();
        assert_ne!(sut, Sut::sample());
    }

    #[test]
    fn from_str_persona() {
        assert!(matches!(
            Sut::from_str("m/44H/1022H/1H/618H/1460H/0H"),
            Err(CommonError::WrongEntityKind {
                expected: CAP26EntityKind::Account,
                found: CAP26EntityKind::Identity
            })
        ))
    }

    #[test]
    fn json_roundtrip() {
        let sut = Sut::sample();

        assert_json_value_eq_after_roundtrip(
            &sut,
            json!("m/44H/1022H/1H/525H/1460H/0H"),
        );
        assert_json_roundtrip(&sut);
    }

    #[test]
    fn json_fails_for_invalid() {
        assert_json_value_fails::<Sut>(json!(""));
        assert_json_value_fails::<Sut>(json!("foobar"));
        assert_json_value_fails::<Sut>(json!("^"));
        assert_json_value_fails::<Sut>(json!("S"));
        assert_json_value_fails::<Sut>(json!("2"));
        assert_json_value_fails::<Sut>(json!("2'"));
        assert_json_value_fails::<Sut>(json!("2X"));
        assert_json_value_fails::<Sut>(json!("   "));
    }

    #[test]
    fn is_network_aware() {
        assert_eq!(
            Sut::new(
                NetworkID::Stokenet,
                CAP26KeyKind::sample(),
                Hardened::sample()
            )
            .network_id(),
            NetworkID::Stokenet
        );
    }

    #[test]
    fn is_security_aware_unsecurified() {
        assert!(!Sut::new(
            NetworkID::Stokenet,
            CAP26KeyKind::sample(),
            Hardened::sample()
        )
        .is_securified(),);
    }

    #[test]
    fn is_security_aware_securified() {
        assert!(Sut::new(
            NetworkID::Stokenet,
            CAP26KeyKind::sample(),
            Hardened::sample_other()
        )
        .is_securified());
    }

    #[test]
    fn entity_kind() {
        assert_eq!(Sut::entity_kind(), CAP26EntityKind::Account);
    }

    #[test]
    fn get_entity_kind() {
        assert_eq!(Sut::sample().get_entity_kind(), CAP26EntityKind::Account);
    }
}

/*
#[cfg(test)]
mod old_sargon_tests {
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

*/
