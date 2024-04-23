use crate::prelude::*;

#[derive(Debug, Deserialize, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionAccountsRequestItem {
    pub number_of_accounts: RequestedQuantity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge: Option<Exactly32Bytes>,
}

impl DappToWalletInteractionAccountsRequestItem {
    pub fn new(
        number_of_accounts: RequestedQuantity,
        challenge: impl Into<Option<Exactly32Bytes>>,
    ) -> Self {
        Self {
            number_of_accounts,
            challenge: challenge.into(),
        }
    }
}

impl HasSampleValues for DappToWalletInteractionAccountsRequestItem {
    fn sample() -> Self {
        Self::new(RequestedQuantity::sample(), Exactly32Bytes::sample())
    }

    fn sample_other() -> Self {
        Self::new(
            RequestedQuantity::sample_other(),
            Exactly32Bytes::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionAccountsRequestItem;

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
