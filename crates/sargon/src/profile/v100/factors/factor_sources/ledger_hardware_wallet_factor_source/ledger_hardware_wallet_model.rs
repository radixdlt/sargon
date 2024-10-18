use crate::prelude::*;

/// The model of a Ledger HQ hardware wallet NanoS, e.g.
/// *Ledger Nano S+*.
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
#[serde(rename_all = "camelCase")]
pub enum LedgerHardwareWalletModel {
    NanoS,

    #[serde(rename = "nanoS+")]
    NanoSPlus,
    NanoX,
}

impl FromStr for LedgerHardwareWalletModel {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self> {
        s.deserialize().map_err(|_| {
            CommonError::InvalidLedgerHardwareWalletModel {
                bad_value: s.to_owned(),
            }
        })
    }
}

impl std::fmt::Display for LedgerHardwareWalletModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.serialize_to_string())
    }
}

impl HasSampleValues for LedgerHardwareWalletModel {
    fn sample() -> Self {
        Self::NanoSPlus
    }

    fn sample_other() -> Self {
        Self::NanoX
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = LedgerHardwareWalletModel;

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
    fn string_roundtrip() {
        use LedgerHardwareWalletModel::*;
        let eq = |f: SUT, s| {
            assert_eq!(f.to_string(), s);
            assert_eq!(SUT::from_str(s).unwrap(), f);
        };

        eq(NanoSPlus, "nanoS+");
        eq(NanoX, "nanoX");
        eq(NanoS, "nanoS");
    }

    #[test]
    fn from_str_err() {
        let s = "invalid ledger hardware model kind!";
        assert_eq!(
            SUT::from_str(s),
            Err(CommonError::InvalidLedgerHardwareWalletModel {
                bad_value: s.to_owned(),
            })
        );
    }

    #[test]
    fn hash() {
        assert_eq!(
            BTreeSet::from_iter([SUT::NanoS, SUT::NanoS].into_iter()).len(),
            1
        );
    }

    #[test]
    fn ord() {
        assert!(SUT::NanoS < SUT::NanoX);
    }

    #[test]
    fn json_roundtrip() {
        assert_json_value_eq_after_roundtrip(&SUT::NanoS, json!("nanoS"));
        assert_json_value_eq_after_roundtrip(&SUT::NanoSPlus, json!("nanoS+"));
        assert_json_value_eq_after_roundtrip(&SUT::NanoX, json!("nanoX"));
    }
}
