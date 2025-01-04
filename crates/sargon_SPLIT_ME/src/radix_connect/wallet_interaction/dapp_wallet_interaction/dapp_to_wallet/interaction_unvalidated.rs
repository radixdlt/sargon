use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
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

impl DappToWalletInteractionUnvalidated {
    pub fn new_from_json_string(
        json_str: impl AsRef<str>,
    ) -> Result<DappToWalletInteractionUnvalidated> {
        let json_str = json_str.as_ref();
        serde_json::from_str(json_str)
            .map_failed_to_deserialize_string::<Self>(json_str)
    }
}

impl DappToWalletInteractionUnvalidated {
    pub fn to_json_string(&self, pretty_printed: bool) -> String {
        if pretty_printed {
            serde_json::to_string_pretty(self)
        } else {
            serde_json::to_string(self)
        }
        .expect("Should always be able to JSON encode DappToWalletInteractionUnvalidated.")
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

impl DappToWalletInteractionUnvalidated {
    pub fn sample_with_interaction_id(
        interaction_id: WalletInteractionId,
    ) -> Self {
        Self::new(
            interaction_id,
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
