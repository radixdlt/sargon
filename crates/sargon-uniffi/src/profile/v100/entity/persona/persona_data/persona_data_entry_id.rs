use crate::prelude::*;
use sargon::PersonaDataEntryID as InternalPersonaDataEntryID;

uniffi::custom_newtype!(PersonaDataEntryID, Uuid);

/// An ID of some PersonaData Entry a user has shared.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion)]
pub struct PersonaDataEntryID(pub Uuid);
