use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WalletToDappInteractionSuccessResponse {
    pub interaction_id: WalletInteractionId,
    pub items: WalletToDappInteractionResponseItems,
}

impl WalletToDappInteractionSuccessResponse {
    pub fn new(
        interaction_id: WalletInteractionId,
        items: WalletToDappInteractionResponseItems,
    ) -> Self {
        Self {
            interaction_id,
            items,
        }
    }
}

impl HasSampleValues for WalletToDappInteractionSuccessResponse {
    fn sample() -> Self {
        Self::new(
            WalletInteractionId::sample(),
            WalletToDappInteractionResponseItems::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            WalletInteractionId::sample_other(),
            WalletToDappInteractionResponseItems::sample_other(),
        )
    }
}

impl WalletToDappInteractionSuccessResponse {
    pub fn sample_with_id(interaction_id: WalletInteractionId) -> Self {
        Self::new(
            interaction_id,
            WalletToDappInteractionResponseItems::sample(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = WalletToDappInteractionSuccessResponse;

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
