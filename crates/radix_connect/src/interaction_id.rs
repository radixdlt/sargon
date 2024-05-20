use crate::prelude::*;

uniffi::custom_newtype!(WalletInteractionId, Uuid);

#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Ord, PartialOrd, Hash,
)]
pub struct WalletInteractionId(pub Uuid);

impl FromStr for WalletInteractionId {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::from_str(s).map(WalletInteractionId).map_err(|_| {
            CommonError::RadixMobileInvalidInteractionID {
                bad_value: s.to_owned(),
            }
        })
    }
}

impl HasSampleValues for WalletInteractionId {
    fn sample() -> Self {
        WalletInteractionId(Uuid::from_bytes([0xff; 16]))
    }

    fn sample_other() -> Self {
        WalletInteractionId(Uuid::from_bytes([0xde; 16]))
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
            "bad".parse::<SUT>(),
            Err(CommonError::RadixMobileInvalidInteractionID {
                bad_value: "bad".to_owned()
            })
        );
    }
}
