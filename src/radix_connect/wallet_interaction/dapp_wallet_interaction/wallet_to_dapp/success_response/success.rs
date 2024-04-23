use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct WalletToDappInteractionSuccessResponse {
    pub interaction_id: WalletInteractionId,
    pub items: WalletToDappInteractionResponseItems,
}

impl WalletToDappInteractionSuccessResponse {
    pub fn new(
        interaction_id: impl Into<WalletInteractionId>,
        items: WalletToDappInteractionResponseItems,
    ) -> Self {
        Self {
            interaction_id: interaction_id.into(),
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
