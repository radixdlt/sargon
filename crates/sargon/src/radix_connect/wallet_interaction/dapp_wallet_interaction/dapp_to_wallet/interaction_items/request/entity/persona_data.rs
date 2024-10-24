use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionPersonaDataRequestItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_requesting_name: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_requested_email_addresses: Option<RequestedQuantity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_requested_phone_numbers: Option<RequestedQuantity>,
}

impl DappToWalletInteractionPersonaDataRequestItem {
    pub fn new(
        is_requesting_name: impl Into<Option<bool>>,
        number_of_requested_email_addresses: impl Into<Option<RequestedQuantity>>,
        number_of_requested_phone_numbers: impl Into<Option<RequestedQuantity>>,
    ) -> Self {
        Self {
            is_requesting_name: is_requesting_name.into(),
            number_of_requested_email_addresses:
                number_of_requested_email_addresses.into(),
            number_of_requested_phone_numbers:
                number_of_requested_phone_numbers.into(),
        }
    }
}

impl HasSampleValues for DappToWalletInteractionPersonaDataRequestItem {
    fn sample() -> Self {
        Self::new(
            true,
            RequestedQuantity::sample(),
            RequestedQuantity::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            false,
            RequestedQuantity::sample_other(),
            RequestedQuantity::sample_other(),
        )
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
