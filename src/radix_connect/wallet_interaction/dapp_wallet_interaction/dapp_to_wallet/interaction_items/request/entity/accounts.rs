use crate::prelude::*;

#[derive(Debug, Deserialize, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionAccountsRequestItem {
    pub number_of_accounts: RequestedQuantity,
    pub challenge: Option<Exactly32Bytes>,
}

impl HasSampleValues for DappToWalletInteractionAccountsRequestItem {
    fn sample() -> Self {
        Self {
            number_of_accounts: RequestedQuantity::sample(),
            challenge: Some(Exactly32Bytes::sample()),
        }
    }

    fn sample_other() -> Self {
        Self {
            number_of_accounts: RequestedQuantity::sample_other(),
            challenge: Some(Exactly32Bytes::sample_other()),
        }
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
