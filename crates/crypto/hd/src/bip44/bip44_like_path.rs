use crate::prelude::*;

/// Either a canonical BIP44 derivation path like so:
///
/// `m / purpose' / coin_type' / account' / change / address_index`
///
/// Or an Radix Olympia BIP44 "like" path, where the `address_index` accidentally
/// was made hardened, i.e.:
///
/// `m / purpose' / coin_type' / account' / change / address_index'`
///
/// This was a mistake made during implementation of Radix Olympia.
///
/// ```
/// use hierarchical_deterministic::prelude::*;
///
/// let parse = |s: &str| s.parse::<BIP44LikePath>();
///
/// assert!(parse("m/44'/1022'/0'/0/0").is_ok()); // Canonical BIP44
/// assert!(parse("m/44'/1022'/0'/0/0'").is_ok()); // BIP44 like
///
/// assert!(parse("m/44'/1022'/0'/0'/0").is_err());
/// assert!(parse("m/44'/1022'/0'/0'/0'").is_err());
/// assert!(parse("m/44'/0'/0'/0/0'").is_err());
/// ```
#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    SerializeDisplay,
    DeserializeFromStr,
    derive_more::Debug,
    derive_more::Display,
)]
#[display("{}", self.to_cap43_string())]
#[debug("{}", self.to_cap43_string_debug())]
pub struct BIP44LikePath {
    pub account: HDPathComponent,
    pub change: HDPathComponent,
    pub index: HDPathComponent,
}
impl BIP44LikePath {
    /// # Panics
    /// Panics if account is not hardened
    /// Panics if change is hardened
    pub fn with_account_change_and_index(
        account: HDPathComponent,
        change: HDPathComponent,
        index: HDPathComponent,
    ) -> Self {
        assert!(account.is_hardened());
        assert!(!change.is_hardened());
        Self {
            account,
            change,
            index,
        }
    }
    pub fn new(index: HDPathComponent) -> Self {
        Self::with_account_change_and_index(BIP44_ACCOUNT, BIP44_CHANGE, index)
    }
    pub fn is_canonical(&self) -> bool {
        self.index.is_unhardened()
    }
}

impl HasDerivationPathScheme for BIP44LikePath {
    fn derivation_path_scheme() -> DerivationPathScheme {
        DerivationPathScheme::Bip44Olympia
    }
}

impl HasSampleValues for BIP44LikePath {
    fn sample() -> Self {
        Self::new(HDPathComponent::Unsecurified(Unsecurified::Unhardened(
            Unhardened::from_local_key_space(0u32).unwrap(),
        )))
    }
    fn sample_other() -> Self {
        Self::new(HDPathComponent::Unsecurified(Unsecurified::Hardened(
            UnsecurifiedHardened::from_local_key_space(1u32).unwrap(),
        )))
    }
}

impl BIP44LikePath {
    pub fn to_hd_path(&self) -> HDPath {
        let mut path: [HDPathComponent; 5] = [PURPOSE; 5];
        path[1] = COIN_TYPE;
        path[2] = self.account;
        path[3] = self.change;
        path[4] = self.index;
        HDPath::new(Vec::from_iter(path))
    }
    pub const PATH_DEPTH: usize = 5;
}

impl From<BIP44LikePath> for HDPath {
    fn from(path: BIP44LikePath) -> Self {
        path.to_hd_path()
    }
}

impl TryFrom<HDPath> for BIP44LikePath {
    type Error = CommonError;
    fn try_from(path: HDPath) -> Result<Self> {
        let components = path.components();

        if components.len() != Self::PATH_DEPTH {
            return Err(CommonError::InvalidDepthOfBIP44Path {
                expected: Self::PATH_DEPTH as u64,
                found: components.len() as u64,
            });
        }
        if components[0] != PURPOSE {
            return Err(CommonError::BIP44PurposeNotFound {
                bad_value: u32::from(components[0].index_in_local_key_space()),
            });
        }
        if components[1] != COIN_TYPE {
            return Err(CommonError::CoinTypeNotFound {
                bad_value: u32::from(components[1].index_in_local_key_space()),
            });
        }
        let bip44_account = components[2];
        if bip44_account.is_unhardened() {
            return Err(CommonError::InvalidBIP44LikePathAccountWasNotHardened);
        }
        let bip44_change = components[3];

        if bip44_change.is_hardened() {
            return Err(
                CommonError::InvalidBIP44LikePathChangeWasUnexpectedlyHardened,
            );
        }

        let index = components[4];

        Ok(Self::with_account_change_and_index(
            bip44_account,
            bip44_change,
            index,
        ))
    }
}

impl ToCAP43String for BIP44LikePath {
    fn to_cap43_string(&self) -> String {
        self.to_hd_path().to_cap43_string()
    }
    fn to_cap43_string_debug(&self) -> String {
        self.to_hd_path().to_cap43_string_debug()
    }
}
impl FromCAP43String for BIP44LikePath {
    fn from_cap43_string(s: impl AsRef<str>) -> Result<Self> {
        HDPath::from_cap43_string(s).and_then(Self::try_from)
    }
}
impl FromStr for BIP44LikePath {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_cap43_string(s)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = BIP44LikePath;

    #[test]
    #[should_panic]
    fn panics_if_account_is_not_hardened() {
        _ = SUT::with_account_change_and_index(
            HDPathComponent::from_global_key_space(0).unwrap(),
            HDPathComponent::from_global_key_space(0).unwrap(),
            HDPathComponent::from_global_key_space(0).unwrap(),
        )
    }

    #[test]
    #[should_panic]
    fn panics_if_change_is_hardened() {
        _ = SUT::with_account_change_and_index(
            HDPathComponent::from_global_key_space(GLOBAL_OFFSET_HARDENED)
                .unwrap(),
            HDPathComponent::from_global_key_space(GLOBAL_OFFSET_HARDENED)
                .unwrap(),
            HDPathComponent::from_global_key_space(0).unwrap(),
        )
    }

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", SUT::sample()), "m/44H/1022H/0H/0/0");
        assert_eq!(format!("{}", SUT::sample_other()), "m/44H/1022H/0H/0/1H");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", SUT::sample()), "m/44'/1022'/0'/0/0");
        assert_eq!(format!("{:?}", SUT::sample_other()), "m/44'/1022'/0'/0/1'");
    }

    #[test]
    fn to_bip32_path() {
        let sut = SUT::sample();
        assert_eq!(format!("{}", sut), "m/44H/1022H/0H/0/0");
    }

    #[test]
    fn from_str_hardened() {
        let sut = SUT::from_str("m/44H/1022H/0H/0/8H").unwrap();
        assert_eq!(
            sut.index,
            HDPathComponent::Unsecurified(Unsecurified::Hardened(
                UnsecurifiedHardened::from_local_key_space(8u32).unwrap(),
            ))
        );
    }

    #[test]
    fn from_str_hardened_shorthand_syntax() {
        let sut = SUT::from_str("m/44'/1022'/0'/0/8'").unwrap();
        assert_eq!(
            sut.index,
            HDPathComponent::Unsecurified(Unsecurified::Hardened(
                UnsecurifiedHardened::from_local_key_space(8u32).unwrap(),
            ))
        );
    }

    #[test]
    fn from_str_unhardened() {
        let sut = SUT::from_str("m/44H/1022H/0H/0/6").unwrap();
        assert_eq!(
            sut.index,
            HDPathComponent::Unsecurified(Unsecurified::Unhardened(
                Unhardened::from_local_key_space(6u32).unwrap(),
            ))
        );
    }

    #[test]
    fn from_str_unhardened_shorthand_syntax() {
        let sut = SUT::from_str("m/44'/1022'/0'/0/6").unwrap();
        assert_eq!(
            sut.index,
            HDPathComponent::Unsecurified(Unsecurified::Unhardened(
                Unhardened::from_local_key_space(6u32).unwrap(),
            ))
        );
    }

    #[test]
    fn json_roundtrip_unhardened() {
        let sut = SUT::sample();

        assert_json_value_eq_after_roundtrip(&sut, json!("m/44H/1022H/0H/0/0"));
        assert_json_roundtrip(&sut);
    }
    #[test]
    fn json_roundtrip_hardened() {
        let sut = SUT::sample_other();

        assert_json_value_eq_after_roundtrip(
            &sut,
            json!("m/44H/1022H/0H/0/1H"),
        );
        assert_json_roundtrip(&sut);
    }

    #[test]
    fn json_fails_for_invalid() {
        assert_json_value_fails::<SUT>(json!(""));
        assert_json_value_fails::<SUT>(json!("foobar"));
        assert_json_value_fails::<SUT>(json!("^"));
        assert_json_value_fails::<SUT>(json!("S"));
        assert_json_value_fails::<SUT>(json!("2"));
        assert_json_value_fails::<SUT>(json!("2'"));
        assert_json_value_fails::<SUT>(json!("2X"));
        assert_json_value_fails::<SUT>(json!("   "));
    }

    #[test]
    fn string_roundtrip() {
        let str = "m/44H/1022H/0H/0/0H";
        let a: BIP44LikePath = str.parse().unwrap();
        assert_eq!(a.to_string(), str);
    }

    #[test]
    fn string_roundtrip_custom_change_and_account_from_str() {
        let str = "m/44H/1022H/2H/3/4H";
        let a: BIP44LikePath = str.parse().unwrap();
        assert_eq!(a.to_string(), str);
    }

    #[test]
    fn string_roundtrip_custom_change_and_account_to_str() {
        let a = BIP44LikePath::with_account_change_and_index(
            HDPathComponent::from_local_key_space(
                5,
                KeySpace::Unsecurified { is_hardened: true },
            )
            .unwrap(),
            HDPathComponent::from_local_key_space(
                1,
                KeySpace::Unsecurified { is_hardened: false },
            )
            .unwrap(),
            HDPathComponent::from_local_key_space(
                3,
                KeySpace::Unsecurified { is_hardened: true },
            )
            .unwrap(),
        );
        let str = a.to_string();
        assert_eq!(str, "m/44H/1022H/5H/1/3H");
        assert_eq!(BIP44LikePath::from_str(&str).unwrap(), a);
    }

    #[test]
    fn sample() {
        assert_eq!(
            BIP44LikePath::sample_other().to_string(),
            "m/44H/1022H/0H/0/1H"
        );
    }

    #[test]
    fn invalid_depth_1() {
        assert_eq!(
            BIP44LikePath::from_str("m/44H"),
            Err(CommonError::InvalidDepthOfBIP44Path {
                expected: BIP44LikePath::PATH_DEPTH as u64,
                found: 1
            })
        );
    }

    #[test]
    fn invalid_depth_3() {
        assert_eq!(
            BIP44LikePath::from_str("m/44H/1022H/0H"),
            Err(CommonError::InvalidDepthOfBIP44Path {
                expected: BIP44LikePath::PATH_DEPTH as u64,
                found: 3
            })
        );
    }

    #[test]
    fn invalid_account_not_hardened() {
        assert_eq!(
            BIP44LikePath::from_str("m/44H/1022H/0/1/2H"),
            Err(CommonError::InvalidBIP44LikePathAccountWasNotHardened)
        );
    }

    #[test]
    fn invalid_change_was_hardened() {
        assert_eq!(
            BIP44LikePath::from_str("m/44H/1022H/0H/0H/2H"),
            Err(CommonError::InvalidBIP44LikePathChangeWasUnexpectedlyHardened)
        );
    }

    #[test]
    fn invalid_index_not_hardened_is_ok() {
        assert!(BIP44LikePath::from_str("m/44H/1022H/0H/0/0").is_ok());
    }

    #[test]
    fn inequality_different_accounts() {
        let a: BIP44LikePath = "m/44H/1022H/0H/0/0H".parse().unwrap();
        let b: BIP44LikePath = "m/44H/1022H/1H/0/0H".parse().unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn inequality_different_change() {
        let a: BIP44LikePath = "m/44H/1022H/0H/0/0H".parse().unwrap();
        let b: BIP44LikePath = "m/44H/1022H/0H/1/0H".parse().unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn inequality_different_index() {
        let a: BIP44LikePath = "m/44H/1022H/0H/0/0H".parse().unwrap();
        let b: BIP44LikePath = "m/44H/1022H/0H/0/1H".parse().unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn network_id() {
        assert_eq!(SUT::sample().network_id(), NetworkID::Mainnet);
    }
}
