use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct TransactionFee {
    pub summary: FeeSummaryToReview,
    pub locks: FeeLocks,
}

impl TransactionFee {
    pub fn new_from_execution_summary(
        execution_summary: ExecutionSummary,
        signature_count: u16,
        notary_is_signatory: bool,
    ) -> Self {
        let summary = FeeSummaryToReview::new_from_execution_summary(
            execution_summary.clone(),
            signature_count,
            notary_is_signatory,
        );
        let locks = execution_summary.fee_locks.clone();
        Self {
            summary,
            locks,
        }
    }
}
