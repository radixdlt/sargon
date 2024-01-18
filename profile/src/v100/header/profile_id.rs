use crate::prelude::*;

/// A stable and globally unique identifier of a Profile.
#[derive(Serialize, Deserialize, Debug, derive_more::Display, Clone, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct ProfileID(pub(crate) Uuid);
uniffi::custom_newtype!(ProfileID, Uuid);

impl FromStr for ProfileID {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::from_str(s)
            .map(ProfileID)
            .map_err(|_| CommonError::InvalidProfileID(s.to_owned()))
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
    #[test]
    fn equality() {
        assert_eq!(ProfileID::placeholder(), ProfileID::placeholder());
        assert_eq!(
            ProfileID::placeholder_other(),
            ProfileID::placeholder_other()
        );
    }
    #[test]
    fn inequality() {
        assert_ne!(ProfileID::placeholder(), ProfileID::placeholder_other());
    }
}
