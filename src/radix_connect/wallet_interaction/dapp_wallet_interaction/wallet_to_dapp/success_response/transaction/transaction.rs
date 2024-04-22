use crate::prelude::*;

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
pub struct DappWalletInteractionTransactionResponseItems {
    pub send: DappWalletInteractionSendTransactionResponseItem,
}

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappWalletInteractionSendTransactionResponseItem {
    pub transaction_intent_hash: String,
}

impl HasSampleValues for DappWalletInteractionTransactionResponseItems {
    fn sample() -> Self {
        Self {
            send: DappWalletInteractionSendTransactionResponseItem::sample(),
        }
    }
    fn sample_other() -> Self {
        Self {
            send: DappWalletInteractionSendTransactionResponseItem::sample_other(),
        }
    }
}

impl HasSampleValues for DappWalletInteractionSendTransactionResponseItem {
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