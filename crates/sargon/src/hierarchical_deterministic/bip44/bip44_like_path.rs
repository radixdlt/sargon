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
/// extern crate sargon;
/// use sargon::prelude::*;
///
/// fn parse(s: &str) -> Result<BIP44LikePath> {
///    s.parse::<BIP44LikePath>()
/// }
///
/// assert!(parse("m/44'/1022'/0'/0/0").is_ok()); // Canonical BIP44
/// assert!(parse("m/44'/1022'/0'/0/0'").is_ok()); // BIP44 like
///
/// assert_eq!(parse("m/44'/1022'/0'/0'/0"), Err(CommonError::InvalidBIP44LikePathChangeWasUnexpectedlyHardened));
/// assert_eq!(parse("m/44'/1022'/0'/0'/0'"), Err(CommonError::InvalidBIP44LikePathChangeWasUnexpectedlyHardened));
/// assert_eq!(parse("m/44'/0'/0'/0/0'"), Err(CommonError::CoinTypeNotFound { bad_value: 0 }));
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
#[display("{}", self.to_bip32_string())]
#[debug("{}", self.to_bip32_string_debug())]
pub struct BIP44LikePath {
    pub index: HDPathComponent,
}
impl BIP44LikePath {
    pub fn new(index: HDPathComponent) -> Self {
        Self { index }
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
        bip44(self.index)
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
                bad_value: components[0].map_to_global_key_space(),
            });
        }
        if components[1] != COIN_TYPE {
            return Err(CommonError::CoinTypeNotFound {
                bad_value: components[1].map_to_global_key_space(),
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

        Ok(Self::new(index))
    }
}

impl ToBIP32Str for BIP44LikePath {
    fn to_bip32_string(&self) -> String {
        self.to_hd_path().to_bip32_string()
    }
    fn to_bip32_string_debug(&self) -> String {
        self.to_hd_path().to_bip32_string_debug()
    }
}
impl FromBIP32Str for BIP44LikePath {
    fn from_bip32_string(s: impl AsRef<str>) -> Result<Self> {
        HDPath::from_bip32_string(s).and_then(Self::try_from)
    }
}
impl FromStr for BIP44LikePath {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_bip32_string(s)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    type Sut = BIP44LikePath;

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
        assert_eq!(format!("{}", Sut::sample()), "m/44H/1022H/0H/0/0");
        assert_eq!(format!("{}", Sut::sample_other()), "m/44H/1022H/0H/0/1H");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", Sut::sample()), "m/44'/1022'/0'/0/0");
        assert_eq!(format!("{:?}", Sut::sample_other()), "m/44'/1022'/0'/0/1'");
    }

    #[test]
    fn to_bip32_path() {
        let sut = Sut::sample();
        assert_eq!(format!("{}", sut), "m/44H/1022H/0H/0/0");
    }

    #[test]
    fn from_str_hardened() {
        let sut = Sut::from_str("m/44H/1022H/0H/0/8H").unwrap();
        assert_eq!(
            sut.index,
            HDPathComponent::Unsecurified(Unsecurified::Hardened(
                UnsecurifiedHardened::from_local_key_space(8u32).unwrap(),
            ))
        );
    }

    #[test]
    fn from_str_hardened_non_canonical() {
        let sut = Sut::from_str("m/44'/1022'/0'/0/8'").unwrap();
        assert_eq!(
            sut.index,
            HDPathComponent::Unsecurified(Unsecurified::Hardened(
                UnsecurifiedHardened::from_local_key_space(8u32).unwrap(),
            ))
        );
    }

    #[test]
    fn from_str_unhardened() {
        let sut = Sut::from_str("m/44H/1022H/0H/0/6").unwrap();
        assert_eq!(
            sut.index,
            HDPathComponent::Unsecurified(Unsecurified::Unhardened(
                Unhardened::from_local_key_space(6u32).unwrap(),
            ))
        );
    }

    #[test]
    fn from_str_unhardened_non_canonical() {
        let sut = Sut::from_str("m/44'/1022'/0'/0/6").unwrap();
        assert_eq!(
            sut.index,
            HDPathComponent::Unsecurified(Unsecurified::Unhardened(
                Unhardened::from_local_key_space(6u32).unwrap(),
            ))
        );
    }

    #[test]
    fn json_roundtrip_unhardened() {
        let sut = Sut::sample();

        assert_json_value_eq_after_roundtrip(&sut, json!("m/44H/1022H/0H/0/0"));
        assert_json_roundtrip(&sut);
    }
    #[test]
    fn json_roundtrip_hardened() {
        let sut = Sut::sample_other();

        assert_json_value_eq_after_roundtrip(
            &sut,
            json!("m/44H/1022H/0H/0/1H"),
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
}
/*
#[cfg(test)]
mod old_sargon_tests {

    use super::*;

    #[test]
    fn equality() {
        assert_eq!(BIP44LikePath::sample(), BIP44LikePath::sample());
        assert_eq!(
            BIP44LikePath::sample_other(),
            BIP44LikePath::sample_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(BIP44LikePath::sample(), BIP44LikePath::sample_other());
    }

    #[test]
    fn string_roundtrip() {
        let str = "m/44H/1022H/0H/0/0H";
        let a: BIP44LikePath = str.parse().unwrap();
        assert_eq!(a.to_string(), str);
    }

    #[test]
    fn sample() {
        assert_eq!(BIP44LikePath::sample().to_string(), "m/44H/1022H/0H/0/0H");
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
    fn invalid_depth_3_via_hdpath() {
        let hdpath: HDPath = "m/44H/1022H/0H".parse().unwrap();
        assert_eq!(
            BIP44LikePath::try_from(&hdpath),
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
        assert!(a != b);
    }

    #[test]
    fn inequality_different_index() {
        let a: BIP44LikePath = "m/44H/1022H/0H/0/0H".parse().unwrap();
        let b: BIP44LikePath = "m/44H/1022H/0H/0/1H".parse().unwrap();
        assert!(a != b);
    }

    #[test]
    fn json_roundtrip() {
        let str = "m/44H/1022H/0H/0/0H";
        let parsed: BIP44LikePath = str.parse().unwrap();
        assert_json_value_eq_after_roundtrip(&parsed, json!(str));
        assert_json_value_ne_after_roundtrip(
            &parsed,
            json!("m/44H/1022H/0H/0/1H"),
        );
    }

    #[test]
    fn new_with_account() {
        assert_ne!(
            BIP44LikePath::with_account_and_index(1, 0),
            BIP44LikePath::new(0)
        );
    }
}

*/
