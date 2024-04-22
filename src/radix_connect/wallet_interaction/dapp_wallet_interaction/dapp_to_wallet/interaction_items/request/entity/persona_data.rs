use crate::prelude::*;

#[derive(Debug, Deserialize, Serialize, PartialEq, uniffi::Record)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionPersonaDataRequestItem;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
