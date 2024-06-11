use crate::prelude::*;

uniffi::custom_newtype!(WalletInteractionId, String);

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    Hash,
    derive_more::Display,
)]
pub struct WalletInteractionId(pub(crate) String);

impl FromStr for WalletInteractionId {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(CommonError::RadixMobileInvalidInteractionID {
                bad_value: s.to_owned(),
            })
        } else {
            Ok(WalletInteractionId(s.to_owned()))
        }
    }
}

impl HasSampleValues for WalletInteractionId {
    fn sample() -> Self {
        WalletInteractionId(Uuid::from_bytes([0xff; 16]).to_string())
    }

    fn sample_other() -> Self {
        WalletInteractionId(Uuid::from_bytes([0xde; 16]).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = WalletInteractionId;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequafrom_invalid_str() {
        assert_eq!(
            "".parse::<SUT>(),
            Err(CommonError::RadixMobileInvalidInteractionID {
                bad_value: "".to_owned()
            })
        );
    }
}
