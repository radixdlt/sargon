use crate::prelude::*;

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct WalletToDappInteractionSuccessResponse {
    pub interaction_id: WalletInteractionId,
    pub items: WalletToDappInteractionResponseItems,
}

impl HasSampleValues for WalletToDappInteractionSuccessResponse {
    fn sample() -> Self {
        Self {
            interaction_id: WalletInteractionId::sample(),
            items: WalletToDappInteractionResponseItems::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            interaction_id: WalletInteractionId::sample_other(),
            items: WalletToDappInteractionResponseItems::sample_other(),
        }
    }
}
