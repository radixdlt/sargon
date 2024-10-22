use crate::prelude::*;

/// This is the result of the transaction analysis.
/// It contains all the information needed to compute and display the transaction details to the user.
#[derive(Clone, Debug, PartialEq, uniffi::Enum)]
pub enum TransactionToReview {
    /// This is the result of the transaction v1 analysis.
    V1 {
        transaction_manifest: TransactionManifest,
        execution_summary: ExecutionSummary,
    },
    /// This is the result of the transaction v2 analysis.
    V2 {
        transaction_manifest: TransactionManifestV2,
        summary: TransactionToReviewV2Summary,
    },
}
