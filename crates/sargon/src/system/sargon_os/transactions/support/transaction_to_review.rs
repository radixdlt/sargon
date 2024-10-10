use crate::prelude::*;

/// This is the result of the transaction preview analysis.
/// It contains all the information needed to compute and display the transaction details to the user.
#[derive(Debug, PartialEq, uniffi::Record)]
pub struct TransactionToReview {
    pub transaction_manifest: TransactionManifest,
    pub execution_summary: ExecutionSummary,
}
