use crate::prelude::*;

#[derive(Debug, Deserialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionPersonaDataRequestItem {
    pub is_requesting_name: Option<bool>,
    pub number_of_requested_email_addresses: Option<RequestedQuantity>,
    pub number_of_requested_phone_numbers: Option<RequestedQuantity>,
}

impl HasSampleValues for DappToWalletInteractionPersonaDataRequestItem {
    fn sample() -> Self {
        Self {
            is_requesting_name: Some(true),
            number_of_requested_email_addresses: Some(
                RequestedQuantity::sample(),
            ),
            number_of_requested_phone_numbers: Some(RequestedQuantity::sample()),
        }
    }

    fn sample_other() -> Self {
        Self {
            is_requesting_name: Some(false),
            number_of_requested_email_addresses: Some(
                RequestedQuantity::sample_other(),
            ),
            number_of_requested_phone_numbers: Some(
                RequestedQuantity::sample_other(),
            ),
        }
    }
}
