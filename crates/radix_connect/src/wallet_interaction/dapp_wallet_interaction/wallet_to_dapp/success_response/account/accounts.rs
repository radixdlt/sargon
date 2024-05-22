use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, uniffi::Record)]
pub struct WalletToDappInteractionAccountsRequestResponseItem {
    pub accounts: Vec<WalletInteractionWalletAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge: Option<Exactly32Bytes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proofs: Option<Vec<WalletToDappInteractionAccountProof>>,
}

impl WalletToDappInteractionAccountsRequestResponseItem {
    pub fn new(
        accounts: Vec<WalletInteractionWalletAccount>,
        challenge: impl Into<Option<Exactly32Bytes>>,
        proofs: impl Into<Option<Vec<WalletToDappInteractionAccountProof>>>,
    ) -> Self {
        Self {
            accounts,
            challenge: challenge.into(),
            proofs: proofs.into(),
        }
    }
}

impl HasSampleValues for WalletToDappInteractionAccountsRequestResponseItem {
    fn sample() -> Self {
        Self::new(
            vec![WalletInteractionWalletAccount::sample()],
            Exactly32Bytes::sample(),
            vec![WalletToDappInteractionAccountProof::sample()],
        )
    }

    fn sample_other() -> Self {
        Self::new(
            vec![WalletInteractionWalletAccount::sample_other()],
            Exactly32Bytes::sample_other(),
            vec![WalletToDappInteractionAccountProof::sample_other()],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = WalletToDappInteractionAccountsRequestResponseItem;

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
