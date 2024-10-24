use crate::prelude::*;

/// A Radix Babylon [BIP-32][bip32] path used to derive identities for Personas, for example `m/44'/1022'/1'/618'/1460'/2'`.
///
/// Internally at RDX Works known as a CAP26 path - [see Confuence][cap26].
///
/// This comes from the general derivation pattern for Radix addresses according to the [SLIP-10][slip10]
/// derivation scheme. In the [SLIP-10][slip10] derivation scheme, every level must be hardened, which
/// is denoted by the `'` or `H` suffix. The official Radix wallet uses 6 levels:
///
/// ```text
/// m / purpose' / coin_type' / network' / entity_kind' / key_kind' / entity_index'
/// ```
///
/// The `IdentityPath` struct is parametrized by Radix network id and account index, but fixes the other
/// constants in the path as follows:
///
/// ```text
/// m / 44' / 1022' / NETWORK_ID' / 618' / 1460' / ACCOUNT_INDEX'
/// ```
///
/// More generally:
/// * `purpose` is fixed as `44` as per [BIP-44][bip44].
/// * `coin_type` is fixed as `1022` for Radix as per [SLIP-0044][slip44].
/// * `network` is the Radix network id (1 for `mainnet`, 2 for `stokenet`, ...).
/// * `entity_kind` is the type of Radix entity which keys are being generated for. Possible values include:
///   * 525 - Pre-allocated [accounts][account].
///   * 618 - Pre-allocated [identities][identity], which are used for [ROLA][rola] for personas.
/// * `key_kind` is the type of key. Possible values include:
///   * 1460 - Transaction Signing (the default).
///   * 1678 - Authentication Signing such as [ROLA][rola]. This is used if a separate key is
///     created for ROLA and stored in account metadata.
/// * `entity_index` is the 0-based index of the particular entity which is being derived.
///
/// See `test_asciisum` for the source of the `entity_kind` and `key_kind` numbers.
///
/// ```
/// extern crate sargon;
/// use sargon::prelude::*;
///
/// assert!("m/44'/1022'/1'/618'/1460'/1'".parse::<IdentityPath>().is_ok());
/// assert!("m/44H/1022H/1H/618H/1460H/1H".parse::<IdentityPath>().is_ok());
/// ```
///
/// [cap26]: https://radixdlt.atlassian.net/wiki/x/aoC4r
/// [bip32]: https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki
/// [bip44]: https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki
/// [slip10]: https://github.com/satoshilabs/slips/blob/master/slip-0010.md
/// [slip44]: https://github.com/satoshilabs/slips/blob/master/slip-0044.md
/// [rola]: https://docs.radixdlt.com/docs/rola-radix-off-ledger-auth
/// [account]: https://docs.radixdlt.com/docs/account
/// [identity]: https://docs.radixdlt.com/docs/identity
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
    pub network_id: NetworkID,
    pub key_kind: CAP26KeyKind,
    pub index: Hardened,
}

impl HasKeyKind for IdentityPath {
    fn key_kind(&self) -> CAP26KeyKind {
        self.key_kind
    }
}

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

    #[test]
    fn inequality_different_index() {
        let a: IdentityPath = "m/44H/1022H/1H/618H/1460H/0H".parse().unwrap();
        let b: IdentityPath = "m/44H/1022H/1H/618H/1460H/1H".parse().unwrap();
        assert_ne!(a, b);
    }
    #[test]
    fn inequality_different_network_id() {
        let a: IdentityPath = "m/44H/1022H/1H/618H/1460H/0H".parse().unwrap();
        let b: IdentityPath = "m/44H/1022H/2H/618H/1460H/0H".parse().unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn inequality_different_key_kind() {
        let a: IdentityPath = "m/44H/1022H/1H/618H/1460H/0H".parse().unwrap();
        let b: IdentityPath = "m/44H/1022H/1H/618H/1678H/0H".parse().unwrap();
        assert_ne!(a, b);
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
}
