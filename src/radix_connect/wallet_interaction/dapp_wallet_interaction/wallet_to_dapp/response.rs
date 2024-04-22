use crate::prelude::*;

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