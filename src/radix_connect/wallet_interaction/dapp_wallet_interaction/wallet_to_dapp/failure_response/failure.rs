use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct WalletToDappInteractionFailureResponse {
    pub interaction_id: WalletInteractionId,
    pub error: DappWalletInteractionErrorType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl WalletToDappInteractionFailureResponse {
    pub fn new(
        interaction_id: WalletInteractionId,
        error: DappWalletInteractionErrorType,
        message: impl Into<Option<String>>,
    ) -> Self {
        Self {
            interaction_id,
            error,
            message: message.into(),
        }
    }
}

impl HasSampleValues for WalletToDappInteractionFailureResponse {
    fn sample() -> Self {
        Self::new(
            WalletInteractionId::sample(),
            DappWalletInteractionErrorType::sample(),
            "sample1".to_owned(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            WalletInteractionId::sample_other(),
            DappWalletInteractionErrorType::sample_other(),
            "sample2".to_owned(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = WalletToDappInteractionFailureResponse;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
