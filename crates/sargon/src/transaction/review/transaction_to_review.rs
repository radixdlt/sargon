use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct TransactionToReview {
    pub transaction_manifest: TransactionManifest,
    pub execution_summary: ExecutionSummary,
}
