use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionUnvalidated {
    pub interaction_id: WalletInteractionId,
    pub items: DappToWalletInteractionItems,
    pub metadata: DappToWalletInteractionMetadataUnvalidated,
}

impl DappToWalletInteractionUnvalidated {
    pub fn new(
        interaction_id: WalletInteractionId,
        items: DappToWalletInteractionItems,
        metadata: DappToWalletInteractionMetadataUnvalidated,
    ) -> Self {
        Self {
            interaction_id,
            items,
            metadata,
        }
    }
}

impl HasSampleValues for DappToWalletInteractionUnvalidated {
    fn sample() -> Self {
        Self::new(
            WalletInteractionId::sample(),
            DappToWalletInteractionItems::sample(),
            DappToWalletInteractionMetadataUnvalidated::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            WalletInteractionId::sample_other(),
            DappToWalletInteractionItems::sample_other(),
            DappToWalletInteractionMetadataUnvalidated::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionUnvalidated;

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
