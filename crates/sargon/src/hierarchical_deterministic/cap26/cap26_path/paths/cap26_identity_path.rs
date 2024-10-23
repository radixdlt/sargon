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
pub struct IdentityPath {
    network_id: NetworkID,
    key_kind: CAP26KeyKind,
    index: Hardened,
}

impl HasKeyKind for IdentityPath {
    fn key_kind(&self) -> CAP26KeyKind {
        self.key_kind
    }
}

// impl HasDerivationPathScheme for IdentityPath {
//     fn derivation_path_scheme() -> DerivationPathScheme {
//         DerivationPathScheme::Cap26
//     }
// }

impl IsNetworkAware for IdentityPath {
    fn network_id(&self) -> NetworkID {
        self.network_id
    }
}

impl IsSecurityStateAware for IdentityPath {
    fn is_securified(&self) -> bool {
        self.index.is_securified()
    }
}

impl NewEntityPath for IdentityPath {
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

impl TryFrom<HDPath> for IdentityPath {
    type Error = CommonError;
    fn try_from(path: HDPath) -> Result<Self> {
        UnvalidatedCAP26Path::try_from(path)
            .and_then(Self::try_from_unvalidated)
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

impl From<IdentityPath> for HDPath {
    fn from(identity_path: IdentityPath) -> Self {
        identity_path.to_hd_path()
    }
}

impl IdentityPath {
    pub fn to_hd_path(&self) -> HDPath {
        cap26(
            self.network_id,
            Self::entity_kind(),
            self.key_kind,
            self.index,
        )
    }
}

impl HasEntityKind for IdentityPath {
    fn entity_kind() -> CAP26EntityKind {
        CAP26EntityKind::Identity
    }
}

impl ToBIP32Str for IdentityPath {
    fn to_bip32_string(&self) -> String {
        self.to_hd_path().to_bip32_string()
    }
    fn to_bip32_string_debug(&self) -> String {
        self.to_hd_path().to_bip32_string_debug()
    }
}

impl FromBIP32Str for IdentityPath {
    fn from_bip32_string(s: impl AsRef<str>) -> Result<Self> {
        HDPath::from_bip32_string(s).and_then(Self::try_from)
    }
}
impl FromStr for IdentityPath {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_bip32_string(s)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    type Sut = IdentityPath;

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
        assert_eq!(format!("{}", sut), "m/44H/1022H/1H/618H/1460H/0H");
    }

    #[test]
    fn debug() {
        let sut = Sut::sample();
        assert_eq!(format!("{:?}", sut), "m/44'/1022'/1'/618'/1460'/0'");
    }

    #[test]
    fn to_bip32_path() {
        let sut = Sut::sample();
        assert_eq!(format!("{}", sut), "m/44H/1022H/1H/618H/1460H/0H");
    }

    #[test]
    fn from_str() {
        let sut = Sut::from_str("m/44H/1022H/1H/618H/1460H/0H").unwrap();
        assert_eq!(sut, Sut::sample());
    }

    #[test]
    fn from_str_securified() {
        let sut = Sut::from_str("m/44H/1022H/1H/618H/1460H/0S").unwrap();
        assert_ne!(sut, Sut::sample());
    }

    #[test]
    fn from_str_account() {
        assert!(matches!(
            Sut::from_str("m/44H/1022H/1H/525H/1460H/0H"),
            Err(CommonError::WrongEntityKind {
                expected: CAP26EntityKind::Identity,
                found: CAP26EntityKind::Account
            })
        ))
    }

    #[test]
    fn json_roundtrip() {
        let sut = Sut::sample();

        assert_json_value_eq_after_roundtrip(
            &sut,
            json!("m/44H/1022H/1H/618H/1460H/0H"),
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
        assert_eq!(Sut::entity_kind(), CAP26EntityKind::Identity);
    }

    #[test]
    fn get_entity_kind() {
        assert_eq!(Sut::sample().get_entity_kind(), CAP26EntityKind::Identity);
    }
}
