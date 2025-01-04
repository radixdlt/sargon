use crate::prelude::*;
use sargon::PersonaDataEntryEmailAddress as InternalPersonaDataEntryEmailAddress;

pub type PersonaDataEntryEmailAddress = EmailAddress;

json_string_convertible!(PersonaDataEntryEmailAddress);

#[uniffi::export]
pub fn new_persona_data_entry_email_address_sample(
) -> PersonaDataEntryEmailAddress {
    InternalPersonaDataEntryEmailAddress::sample().into()
}

#[uniffi::export]
pub fn new_persona_data_entry_email_address_sample_other(
) -> PersonaDataEntryEmailAddress {
    InternalPersonaDataEntryEmailAddress::sample_other().into()
}
