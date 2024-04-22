use crate::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappWalletInteractionPersonaDataRequestResponseItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<PersonaDataEntryName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_addresses: Option<Vec<PersonaDataEntryEmailAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<PersonaDataEntryPhoneNumber>>,
}

impl HasSampleValues for DappWalletInteractionPersonaDataRequestResponseItem {
    fn sample() -> Self {
        Self {
            name: Some(PersonaDataEntryName::sample()),
            email_addresses: Some(vec![PersonaDataEntryEmailAddress::sample()]),
            phone_numbers: Some(vec![PersonaDataEntryPhoneNumber::sample()]),
        }
    }

    fn sample_other() -> Self {
        Self {
            name: Some(PersonaDataEntryName::sample_other()),
            email_addresses: Some(vec![PersonaDataEntryEmailAddress::sample_other()]),
            phone_numbers: Some(vec![PersonaDataEntryPhoneNumber::sample_other()]),
        }
    }
}
