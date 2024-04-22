use crate::prelude::*;

#[derive(Debug, Deserialize, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteraction {
    pub interaction_id: WalletInteractionId,
    pub items: DappToWalletInteractionItems,
    pub metadata: DappToWalletInteractionMetadata,
}

impl HasSampleValues for DappToWalletInteraction {
    fn sample() -> Self {
        Self {
            interaction_id: WalletInteractionId::sample(),
            items: DappToWalletInteractionItems::sample(),
            metadata: DappToWalletInteractionMetadata::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            interaction_id: WalletInteractionId::sample_other(),
            items: DappToWalletInteractionItems::sample_other(),
            metadata: DappToWalletInteractionMetadata::sample_other(),
        }
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[allow(clippy::upper_case_acronyms)]
  type SUT = DappToWalletInteraction;

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