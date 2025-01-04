use crate::prelude::*;
use sargon::DappToWalletInteractionPersonaDataRequestItem as InternalDappToWalletInteractionPersonaDataRequestItem;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct DappToWalletInteractionPersonaDataRequestItem {
    pub is_requesting_name: Option<bool>,
    pub number_of_requested_email_addresses: Option<RequestedQuantity>,
    pub number_of_requested_phone_numbers: Option<RequestedQuantity>,
}
