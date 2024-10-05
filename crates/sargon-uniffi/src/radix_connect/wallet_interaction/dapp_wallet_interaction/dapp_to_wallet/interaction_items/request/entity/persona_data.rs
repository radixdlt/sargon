use crate::prelude::*;
use sargon::DappToWalletInteractionPersonaDataRequestItem as InternalDappToWalletInteractionPersonaDataRequestItem;

#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct DappToWalletInteractionPersonaDataRequestItem {
    pub is_requesting_name: Option<bool>,
    pub number_of_requested_email_addresses: Option<RequestedQuantity>,
    pub number_of_requested_phone_numbers: Option<RequestedQuantity>,
}

impl From<InternalDappToWalletInteractionPersonaDataRequestItem> for DappToWalletInteractionPersonaDataRequestItem {
    fn from(value: InternalDappToWalletInteractionPersonaDataRequestItem) -> Self {
        Self {
            is_requesting_name: value.is_requesting_name,
            number_of_requested_email_addresses: value.number_of_requested_email_addresses.map(Into::into),
            number_of_requested_phone_numbers: value.number_of_requested_phone_numbers.map(Into::into),
        }
    }
}

impl Into<InternalDappToWalletInteractionPersonaDataRequestItem> for DappToWalletInteractionPersonaDataRequestItem {
    fn into(self) -> InternalDappToWalletInteractionPersonaDataRequestItem {
        InternalDappToWalletInteractionPersonaDataRequestItem {
            is_requesting_name: self.is_requesting_name,
            number_of_requested_email_addresses: self.number_of_requested_email_addresses.map(Into::into),
            number_of_requested_phone_numbers: self.number_of_requested_phone_numbers.map(Into::into),
        }
    }
}