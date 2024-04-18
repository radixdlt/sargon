use crate::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
#[serde(tag = "discriminator")]
pub enum DappWalletInteractionResponse {
    #[serde(rename = "success")]
    Success(DappWalletInteractionSuccessResponse),
    #[serde(rename = "failure")]
    Failure(DappWalletInteractionFailureResponse),
}

#[derive(Debug, Serialize, PartialEq)]
pub struct DappWalletInteractionTransactionResponseItems {
    pub send: DappWalletInteractionSendTransactionResponseItem,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DappWalletInteractionSendTransactionResponseItem {
    pub transaction_intent_hash: String,
}
