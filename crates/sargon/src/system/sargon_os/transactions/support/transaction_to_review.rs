use crate::prelude::*;

/// This is the result of the transaction preview analysis.
/// It contains all the information needed to compute and display the transaction details to the user.
#[derive(Clone, Debug, PartialEq)]
pub struct TransactionToReview {
    pub transaction_manifest: TransactionManifest,
    pub execution_summary: ExecutionSummary,
}

impl HasSampleValues for TransactionToReview {
    fn sample() -> Self {
        Self {
            transaction_manifest: TransactionManifest::sample(),
            execution_summary: ExecutionSummary::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            transaction_manifest: TransactionManifest::sample_other(),
            execution_summary: ExecutionSummary::sample_other(),
        }
    }
}
