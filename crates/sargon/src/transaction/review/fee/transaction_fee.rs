use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct TransactionFee {
    pub summary: FeeSummaryToReview,
    pub locks: FeeLocks,
    /// The calculation mode of the fee
    pub mode: FeeMode,
}

impl TransactionFee {
    pub fn new_from_execution_summary(
        execution_summary: ExecutionSummary,
        signatures_count: usize,
        notary_is_signatory: bool,
        include_lock_fee: bool,
    ) -> Self {
        Self {
            summary: FeeSummaryToReview::new_from_execution_summary(
                execution_summary.clone(),
                signatures_count,
                notary_is_signatory,
                include_lock_fee,
            ),
            locks: execution_summary.fee_locks.clone(),
            mode: FeeMode::Normal {
                customization: NormalFeeCustomization::new_from_summary(
                    FeeSummaryToReview::new_from_execution_summary(
                        execution_summary.clone(),
                        signatures_count,
                        notary_is_signatory,
                        include_lock_fee,
                    ),
                    execution_summary.fee_locks,
                ),
            },
        }
    }

    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            FeeMode::Normal { customization: _ } => FeeMode::Advanced {
                customization: AdvancedFeeCustomization::new(
                    self.summary.clone(),
                    self.locks.clone(),
                ),
            },
            FeeMode::Advanced { customization: _ } => FeeMode::Normal {
                customization: NormalFeeCustomization::new_from_summary(
                    self.summary.clone(),
                    self.locks.clone(),
                ),
            },
        }
    }

    pub fn is_normal_mode(&self) -> bool {
        matches!(self.mode, FeeMode::Normal { .. })
    }
}
