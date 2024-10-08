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
        let summary = FeeSummaryToReview::new_from_execution_summary(
            execution_summary.clone(),
            signatures_count,
            notary_is_signatory,
            include_lock_fee,
        );
        let locks = execution_summary.fee_locks.clone();
        let mode = FeeMode::Normal {
            customization: NormalFeeCustomization::new_from_summary(
                summary.clone(),
                locks.clone(),
            ),
        };
        Self {
            summary,
            locks,
            mode,
        }
    }

    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            FeeMode::Normal { .. } => FeeMode::Advanced {
                customization: AdvancedFeeCustomization::new(
                    self.summary.clone(),
                    self.locks.clone(),
                ),
            },
            FeeMode::Advanced { .. } => FeeMode::Normal {
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

    pub fn add_lock_fee_cost(&mut self) {
        self.summary.lock_fee_cost = FeeConstants::lock_fee_cost(true);
    }
    
    pub fn update_notarizing_cost(&mut self, notary_is_signatory: bool) {
        self.summary.notarizing_cost = FeeConstants::notarizing_cost(notary_is_signatory);
    }
    
    pub fn update_signatures_cost(&mut self, signature_count: usize) {
        self.summary.signatures_cost = FeeConstants::signatures_cost(signature_count);
    }
}
