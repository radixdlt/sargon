use crate::prelude::*;

#[derive(Debug, Serialize, PartialEq, uniffi::Enum)]
#[serde(tag = "discriminator")]
pub enum WalletToDappInteractionResponse {
    #[serde(rename = "success")]
    Success(WalletToDappInteractionSuccessResponse),
    #[serde(rename = "failure")]
    Failure(WalletToDappInteractionFailureResponse),
}

impl HasSampleValues for WalletToDappInteractionResponse {
    fn sample() -> Self {
        WalletToDappInteractionResponse::Success(
            WalletToDappInteractionSuccessResponse::sample(),
        )
    }
    fn sample_other() -> Self {
        WalletToDappInteractionResponse::Failure(
            WalletToDappInteractionFailureResponse::sample(),
        )
    }
}
