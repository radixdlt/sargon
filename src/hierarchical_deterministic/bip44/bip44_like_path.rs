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
    uniffi::Record,
)]
#[display("{}", self.bip32_string())]
pub struct BIP44LikePath {
    pub path: HDPath,
}

impl BIP44LikePath {
    pub fn from(path: HDPath) -> Self {
        Self { path }
    }
    pub const PATH_DEPTH: usize = 5;

    fn assert_depth_of(path: &HDPath) -> Result<(), CommonError> {
        let found = path.depth();
        if found != Self::PATH_DEPTH {
            return Err(CommonError::InvalidDepthOfBIP44Path {
                expected: Self::PATH_DEPTH as u64,
                found: found as u64,
            });
        }
        Ok(())
    }
}

impl TryFrom<&HDPath> for BIP44LikePath {
    type Error = CommonError;

    fn try_from(value: &HDPath) -> Result<Self> {
        let (path, components) = HDPath::try_parse_base_hdpath(value, |v| {
            CommonError::InvalidDepthOfBIP44Path {
                expected: Self::PATH_DEPTH as u64,
                found: v as u64,
            }
        })?;

        BIP44LikePath::assert_depth_of(value)?;
        let account = &components[2];
        if !account.is_hardened() {
            return Err(CommonError::InvalidBIP44LikePathAccountWasNotHardened);
        }
        let change = &components[3];
        if change.is_hardened() {
            return Err(
                CommonError::InvalidBIP44LikePathChangeWasUnexpectedlyHardened,
            );
        }

        let index = &components[4];
        if !index.is_hardened() {
            return Err(CommonError::InvalidBIP44LikePathIndexWasNotHardened);
        }
        Ok(Self::from(path))
    }
}

impl BIP44LikePath {
    fn with_account_and_index(
        account: HDPathValue,
        index: HDPathValue,
    ) -> Self {
        let c0 = HDPathComponent::bip44_purpose(); // purpose
        let c1 = HDPathComponent::bip44_cointype(); // cointype
        let c2 = HDPathComponent::harden(account); // account
        let c3 = HDPathComponent::non_hardened(0); // change
        let c4 = HDPathComponent::harden(index); // index
        let components = vec![c0, c1, c2, c3, c4];
        let path = HDPath::from_components(components);
        Self::from(path)
    }

    pub fn new(index: HDPathValue) -> Self {
        Self::with_account_and_index(0, index)
    }
}

impl Derivation for BIP44LikePath {
    fn derivation_path(&self) -> DerivationPath {
        DerivationPath::BIP44Like {
            value: self.clone(),
        }
    }
    fn hd_path(&self) -> &HDPath {
        &self.path
    }

    fn scheme(&self) -> DerivationPathScheme {
        DerivationPathScheme::Bip44Olympia
    }
}

impl FromStr for BIP44LikePath {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (path, _) = HDPath::try_parse_base(s, |v| {
            CommonError::InvalidDepthOfBIP44Path {
                expected: Self::PATH_DEPTH as u64,
                found: v as u64,
            }
        })?;
        Self::try_from(&path)
    }
}

impl HasSampleValues for BIP44LikePath {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::from_str("m/44H/1022H/0H/0/0H").expect("Valid sample")
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::from_str("m/44H/1022H/0H/0/1H").expect("Valid sample")
    }
}

#[cfg(test)]
mod tests {

    use crate::prelude::*;

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
    fn invalid_index_not_hardened() {
        assert_eq!(
            BIP44LikePath::from_str("m/44H/1022H/0H/0/0"),
            Err(CommonError::InvalidBIP44LikePathIndexWasNotHardened)
        );
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
