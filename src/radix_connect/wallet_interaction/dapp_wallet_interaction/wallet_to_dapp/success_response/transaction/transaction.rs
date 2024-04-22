use crate::prelude::*;

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
pub struct WalletToDappInteractionTransactionResponseItems {
    pub send: WalletToDappInteractionSendTransactionResponseItem,
}

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct WalletToDappInteractionSendTransactionResponseItem {
    pub transaction_intent_hash: String,
}

impl HasSampleValues for WalletToDappInteractionTransactionResponseItems {
    fn sample() -> Self {
        Self {
            send: WalletToDappInteractionSendTransactionResponseItem::sample(),
        }
    }
    fn sample_other() -> Self {
        Self {
            send:
                WalletToDappInteractionSendTransactionResponseItem::sample_other(
                ),
        }
    }
}

impl HasSampleValues for WalletToDappInteractionSendTransactionResponseItem {
    fn sample() -> Self {
        Self {
            transaction_intent_hash: IntentHash::sample().to_string(),
        }
    }
    fn sample_other() -> Self {
        Self {
            transaction_intent_hash: IntentHash::sample_other().to_string(),
        }
    }
}
