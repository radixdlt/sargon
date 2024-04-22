use crate::prelude::*;

#[derive(Debug, Deserialize, PartialEq, uniffi::Record)]
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
