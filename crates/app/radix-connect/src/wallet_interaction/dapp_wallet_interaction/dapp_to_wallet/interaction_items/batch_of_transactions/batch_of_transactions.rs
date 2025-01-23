use crate::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct DappToWalletInteractionBatchOfTransactions {
    pub transactions: Vec<UnvalidatedTransactionManifest>,
}

impl DappToWalletInteractionBatchOfTransactions {
    pub fn new(
        transactions: impl IntoIterator<Item = UnvalidatedTransactionManifest>,
    ) -> Self {
        Self {
            transactions: transactions.into_iter().collect(),
        }
    }
}

impl HasSampleValues for DappToWalletInteractionBatchOfTransactions {
    fn sample() -> Self {
        Self::new([
            UnvalidatedTransactionManifest::sample(),
            UnvalidatedTransactionManifest::sample_other(),
        ])
    }

    fn sample_other() -> Self {
        Self::new([UnvalidatedTransactionManifest::sample_other()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionBatchOfTransactions;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
