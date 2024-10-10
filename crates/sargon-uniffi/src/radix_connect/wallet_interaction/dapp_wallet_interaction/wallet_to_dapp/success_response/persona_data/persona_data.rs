use crate::prelude::*;
use sargon::WalletToDappInteractionPersonaDataRequestResponseItem as InternalWalletToDappInteractionPersonaDataRequestResponseItem;

#[derive( PartialEq, Clone,  uniffi::Record)]
pub struct WalletToDappInteractionPersonaDataRequestResponseItem {
    pub name: Option<PersonaDataEntryName>,
    pub email_addresses: Option<Vec<PersonaDataEntryEmailAddress>>,
    pub phone_numbers: Option<Vec<PersonaDataEntryPhoneNumber>>,
}

impl From<InternalWalletToDappInteractionPersonaDataRequestResponseItem> for WalletToDappInteractionPersonaDataRequestResponseItem {
    fn from(value: InternalWalletToDappInteractionPersonaDataRequestResponseItem) -> Self {
        Self {
            name: value.name.map(|v| v.into()),
            email_addresses: value.email_addresses.map(|v| v.into_iter().map(|v| v.into()).collect()),
            phone_numbers: value.phone_numbers.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl Into<InternalWalletToDappInteractionPersonaDataRequestResponseItem> for WalletToDappInteractionPersonaDataRequestResponseItem {
    fn into(self) -> InternalWalletToDappInteractionPersonaDataRequestResponseItem {
        InternalWalletToDappInteractionPersonaDataRequestResponseItem {
            name: self.name.map(|v| v.into()),
            email_addresses: self.email_addresses.map(|v| v.into_iter().map(|v| v.into()).collect()),
            phone_numbers: self.phone_numbers.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}