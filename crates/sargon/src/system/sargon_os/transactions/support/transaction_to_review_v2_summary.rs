use crate::prelude::*;

/// This is the summary result of the transaction v2 analysis.
/// Based on the type of the v2 transaction, the analysis result is different.
#[derive(Clone, Debug, PartialEq, uniffi::Enum)]
pub enum TransactionToReviewV2Summary {
    /// This is the result of the static analysis of the transaction.
    Open { manifest_summary: ManifestSummary },
    /// This is the result of the dynamic analysis of the transaction
    Enclosed { execution_summary: ExecutionSummary },
}
