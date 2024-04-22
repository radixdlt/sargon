use crate::prelude::*;

#[derive(Debug, Serialize, PartialEq, Clone, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct WalletToDappInteractionPersonaDataRequestResponseItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<PersonaDataEntryName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_addresses: Option<Vec<PersonaDataEntryEmailAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<PersonaDataEntryPhoneNumber>>,
}

impl HasSampleValues for WalletToDappInteractionPersonaDataRequestResponseItem {
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
            email_addresses: Some(vec![
                PersonaDataEntryEmailAddress::sample_other(),
            ]),
            phone_numbers: Some(vec![
                PersonaDataEntryPhoneNumber::sample_other(),
            ]),
        }
    }
}
