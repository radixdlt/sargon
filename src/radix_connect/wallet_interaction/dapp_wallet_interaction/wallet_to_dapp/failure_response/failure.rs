use crate::prelude::*;

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct WalletToDappInteractionFailureResponse {
    pub interaction_id: WalletInteractionId,
    pub error: DappWalletInteractionErrorType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl HasSampleValues for WalletToDappInteractionFailureResponse {
    fn sample() -> Self {
        Self {
            interaction_id: WalletInteractionId::sample(),
            error: DappWalletInteractionErrorType::sample(),
            message: Some("sample1".to_string()),
        }
    }

    fn sample_other() -> Self {
        Self {
            interaction_id: WalletInteractionId::sample_other(),
            error: DappWalletInteractionErrorType::sample_other(),
            message: Some("sample2".to_string()),
        }
    }
}
