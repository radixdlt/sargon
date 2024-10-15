use crate::prelude::*;
use sargon::WalletToDappInteractionPersonaDataRequestResponseItem as InternalWalletToDappInteractionPersonaDataRequestResponseItem;

#[derive(PartialEq, Clone, InternalConversionV2, uniffi::Record)]
pub struct WalletToDappInteractionPersonaDataRequestResponseItem {
    pub name: Option<PersonaDataEntryName>,
    pub email_addresses: Option<Vec<PersonaDataEntryEmailAddress>>,
    pub phone_numbers: Option<Vec<PersonaDataEntryPhoneNumber>>,
}
