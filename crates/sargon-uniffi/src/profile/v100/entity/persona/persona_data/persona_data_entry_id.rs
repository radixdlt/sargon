use crate::prelude::*;
use sargon::PersonaDataEntryID as InternalPersonaDataEntryID;

/// An ID of some PersonaData Entry a user has shared.
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    uniffi::Record,
)]
pub struct PersonaDataEntryID {
    pub value: Uuid,
}

impl From<InternalPersonaDataEntryID> for PersonaDataEntryID {
    fn from(value: InternalPersonaDataEntryID) -> Self {
        Self {
            value: value.0,
        }
    }
}

impl Into<InternalPersonaDataEntryID> for PersonaDataEntryID {
    fn into(self) -> InternalPersonaDataEntryID {
        Self(self.value)
    }
}