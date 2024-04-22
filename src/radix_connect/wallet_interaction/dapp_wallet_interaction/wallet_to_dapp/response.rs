use crate::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, uniffi::Enum)]
#[serde(tag = "discriminator")]
pub enum DappWalletInteractionResponse {
    #[serde(rename = "success")]
    Success(DappWalletInteractionSuccessResponse),
    #[serde(rename = "failure")]
    Failure(DappWalletInteractionFailureResponse),
}

impl HasSampleValues for DappWalletInteractionResponse {
    fn sample() -> Self {
        DappWalletInteractionResponse::Success(DappWalletInteractionSuccessResponse::sample())
    }
    fn sample_other() -> Self {
        DappWalletInteractionResponse::Failure(DappWalletInteractionFailureResponse::sample())
    }
}

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
pub struct DappWalletInteractionTransactionResponseItems {
    pub send: DappWalletInteractionSendTransactionResponseItem,
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

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappWalletInteractionSendTransactionResponseItem {
    pub transaction_intent_hash: String,
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