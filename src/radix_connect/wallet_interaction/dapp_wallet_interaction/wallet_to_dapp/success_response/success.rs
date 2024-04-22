use crate::prelude::*;

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappWalletInteractionSuccessResponse {
    pub interaction_id: WalletInteractionId,
    pub items: DappWalletInteractionResponseItems,
}

impl HasSampleValues for DappWalletInteractionSuccessResponse {
    fn sample() -> Self {
        Self {
            interaction_id: WalletInteractionId::sample(),
            items: DappWalletInteractionResponseItems::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            interaction_id: WalletInteractionId::sample_other(),
            items: DappWalletInteractionResponseItems::sample_other(),
        }
    }
}