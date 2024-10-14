use crate::prelude::*;
use sargon::SharedPersonaData as InternalSharedPersonaData;

/// Identities for PersonaData entry values a user have shared with a dApp.
#[derive(Clone, PartialEq, Hash, Eq, InternalConversionV2, uniffi::Record)]
pub struct SharedPersonaData {
    /// ID of a `PersonaDataEntryName` the user has shared with some dApp on some network,
    /// can be `None`.
    pub name: Option<PersonaDataEntryID>,

    /// IDs of a `PersonaDataEntryEmailAddress`es the user has shared with some dApp on some network
    /// can be `None`, or can be `Some(<EMPTY>)`.
    pub email_addresses: Option<SharedToDappWithPersonaIDsOfPersonaDataEntries>,

    /// IDs of a `PersonaDataEntryPhoneNumber`s the user has shared with some dApp on some network
    /// can be `None`, or can be `Some(<EMPTY>)`.
    pub phone_numbers: Option<SharedToDappWithPersonaIDsOfPersonaDataEntries>,
}

#[uniffi::export]
pub fn new_shared_persona_data_sample() -> SharedPersonaData {
    InternalSharedPersonaData::sample().into()
}

#[uniffi::export]
pub fn new_shared_persona_data_sample_other() -> SharedPersonaData {
    InternalSharedPersonaData::sample_other().into()
}
