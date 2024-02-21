use crate::prelude::*;

/// A stable and globally unique identifier of a Profile.
#[derive(
    Serialize,
    Deserialize,
    Debug,
    derive_more::Display,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
#[serde(transparent)]
pub struct ProfileID(pub(crate) Uuid);
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

impl HasPlaceholder for ProfileID {
    fn placeholder() -> Self {
        ProfileID(Uuid::from_bytes([0xff; 16]))
    }

    fn placeholder_other() -> Self {
        ProfileID(Uuid::from_bytes([0xde; 16]))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ProfileID;

    #[test]
    fn equality() {
        assert_eq!(SUT::placeholder(), SUT::placeholder());
        assert_eq!(SUT::placeholder_other(), SUT::placeholder_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::placeholder(), SUT::placeholder_other());
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
}
