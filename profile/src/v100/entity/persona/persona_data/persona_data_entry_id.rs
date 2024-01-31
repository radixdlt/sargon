use crate::prelude::*;

// Generate the FfiConverter needed by UniFFI for newtype `PersonaDataEntryID`.
uniffi::custom_newtype!(PersonaDataEntryID, Uuid);

/// An ID of some PersonaData Entry a user has shared.
#[derive(
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Eq,
    derive_more::Display,
    derive_more::Debug,
    Hash,
)]
#[serde(transparent)]
pub struct PersonaDataEntryID(pub Uuid); // Unfortunately we have to declare this since we want it to impl `Identifiable`, which is not our trait and `Uuid` is not our type... => force to have a newtype.

impl Identifiable for PersonaDataEntryID {
    type ID = Self;

    fn id(&self) -> Self::ID {
        self.clone()
    }
}

impl std::ops::Deref for PersonaDataEntryID {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PersonaDataEntryID {
    /// Generates a new `PersonaDataEntryID` (using Uuid::new_v4())
    pub fn generate() -> Self {
        id().into()
    }

    /// Should only be used by unit tests and placeholder values
    pub(crate) fn nil() -> Self {
        Uuid::nil().into()
    }
}

impl FromStr for PersonaDataEntryID {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::from_str(s)
            .map_err(|_| CommonError::InvalidUUIDv4(s.to_owned()))
            .map(|v| v.into())
    }
}

impl From<Uuid> for PersonaDataEntryID {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl HasPlaceholder for PersonaDataEntryID {
    fn placeholder() -> Self {
        Uuid::from_str("00000000-0000-0000-0000-000000000001")
            .expect("Should have a valid placeholder")
            .into()
    }

    fn placeholder_other() -> Self {
        Uuid::from_str("00000000-0000-0000-0000-000000000002")
            .expect("Should have a valid placeholder")
            .into()
    }
}
