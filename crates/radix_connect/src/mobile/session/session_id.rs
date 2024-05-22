use crate::prelude::*;

uniffi::custom_newtype!(SessionID, Uuid);

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
pub struct SessionID(pub Uuid);

impl FromStr for SessionID {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::from_str(s).map(SessionID).map_err(|_| {
            CommonError::RadixConnectMobileInvalidSessionID {
                bad_value: s.to_owned(),
            }
        })
    }
}

impl HasSampleValues for SessionID {
    fn sample() -> Self {
        SessionID(Uuid::from_bytes([0xff; 16]))
    }

    fn sample_other() -> Self {
        SessionID(Uuid::from_bytes([0xde; 16]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SessionID;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequafrom_invalid_str() {
        assert_eq!(
            "bad".parse::<SUT>(),
            Err(CommonError::RadixConnectMobileInvalidSessionID {
                bad_value: "bad".to_owned()
            })
        );
    }
}
