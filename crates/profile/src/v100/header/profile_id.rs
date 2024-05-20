use crate::prelude::*;

/// A stable and globally unique identifier of a Profile.
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Copy,
    derive_more::Display,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
#[serde(transparent)]
pub struct ProfileID(pub Uuid);
uniffi::custom_newtype!(ProfileID, Uuid);

impl FromStr for ProfileID {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::from_str(s).map(ProfileID).map_err(|_| {
            CommonError::InvalidProfileID {
                bad_value: s.to_owned(),
            }
        })
    }
}

impl HasSampleValues for ProfileID {
    fn sample() -> Self {
        ProfileID(Uuid::sample())
    }

    fn sample_other() -> Self {
        ProfileID(Uuid::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ProfileID;

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
    fn from_str_invalid() {
        assert_eq!(
            "bad".parse::<SUT>(),
            Err(CommonError::InvalidProfileID {
                bad_value: "bad".to_owned()
            })
        );
    }

    #[test]
    fn from_upper_case_is_ok() {
        assert!(SUT::from_str("66F07CA2-A9D9-49E5-8152-77ACA3D1DD74").is_ok())
    }
}
