use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "discriminator")]
#[allow(clippy::large_enum_variant)]
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

impl WalletToDappInteractionResponse {
    pub fn is_success(&self) -> bool {
        match self {
            WalletToDappInteractionResponse::Success(_) => true,
            WalletToDappInteractionResponse::Failure(_) => false,
        }
    }

    pub fn interaction_id(&self) -> WalletInteractionId {
        match self {
            WalletToDappInteractionResponse::Success(response) => {
                response.interaction_id.clone()
            }
            WalletToDappInteractionResponse::Failure(response) => {
                response.interaction_id.clone()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = WalletToDappInteractionResponse;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn is_success() {
        assert!(SUT::sample().is_success());
        assert!(!SUT::sample_other().is_success());
    }
}
