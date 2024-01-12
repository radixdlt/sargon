use crate::prelude::*;

/// A stable and globally unique identifier of a Profile.
#[derive(Serialize, Deserialize, Debug, derive_more::Display, Clone, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct ProfileID(pub(crate) Uuid);
uniffi::custom_newtype!(ProfileID, Uuid);
impl ProfileID {
    pub fn from_str(s: &str) -> Result<Self> {
        Uuid::from_str(s)
            .map(ProfileID)
            .map_err(|_| CommonError::InvalidProfileID(s.to_owned()))
    }
}
