use crate::prelude::*;
use sargon::SharedPersonaData as InternalSharedPersonaData;

/// Identities for PersonaData entry values a user have shared with a dApp.
#[derive(
    Clone,
    Default,
    
    PartialEq,
    Hash,
    Eq,
     uniffi::Record,
)]
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

impl From<InternalSharedPersonaData> for SharedPersonaData {
    fn from(value: InternalSharedPersonaData) -> Self {
        Self {
            name: value.name.map(Into::into),
            email_addresses: value.email_addresses.map(Into::into),
            phone_numbers: value.phone_numbers.map(Into::into),
        }
    }
}

impl Into<InternalSharedPersonaData> for SharedPersonaData {
    fn into(self) -> InternalSharedPersonaData {
        InternalSharedPersonaData {
            name: self.name.map(Into::into),
            email_addresses: self.email_addresses.map(Into::into),
            phone_numbers: self.phone_numbers.map(Into::into),
        }
    }
}

#[uniffi::export]
pub fn new_shared_persona_data_sample() -> SharedPersonaData {
    InternalSharedPersonaData::sample().into()
}

#[uniffi::export]
pub fn new_shared_persona_data_sample_other() -> SharedPersonaData {
    InternalSharedPersonaData::sample_other().into()
}

