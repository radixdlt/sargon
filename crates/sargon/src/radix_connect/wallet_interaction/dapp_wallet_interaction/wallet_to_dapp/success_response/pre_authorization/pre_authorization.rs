use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct WalletToDappInteractionPreAuthorizationResponseItems {
    pub signed_partial_transaction: SignedPartialTransaction,
}

impl WalletToDappInteractionPreAuthorizationResponseItems {
    pub fn new(signed_partial_transaction: SignedPartialTransaction) -> Self {
        Self {
            signed_partial_transaction,
        }
    }
}

impl HasSampleValues for WalletToDappInteractionPreAuthorizationResponseItems {
    fn sample() -> Self {
        Self::new(SignedPartialTransaction::sample())
    }

    fn sample_other() -> Self {
        Self::new(SignedPartialTransaction::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = WalletToDappInteractionPreAuthorizationResponseItems;

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
