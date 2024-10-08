use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct AdvancedFeeCustomization {
    pub padding_fee: Decimal192,
    pub tip_percentage: u16,
    pub paid_by_dapps: Decimal192,
    total_execution_cost: Decimal192,
    fee_summary: FeeSummaryToReview,
}

impl AdvancedFeeCustomization {
    pub fn new(
        fee_summary_to_review: FeeSummaryToReview,
        fee_locks: FeeLocks,
    ) -> Self {
        let total_execution_cost = fee_summary_to_review.total_execution_cost();
        let padding_fee = (total_execution_cost
            + fee_summary_to_review.summary.finalization_cost
            + fee_summary_to_review.summary.storage_expansion_cost)
            * FeeConstants::network_fee_multiplier();

        Self {
            padding_fee: padding_fee,
            tip_percentage: 0,
            // non-contingent lock will pay for some of the fee.
            paid_by_dapps: -fee_locks.lock,
            total_execution_cost: total_execution_cost,
            fee_summary: fee_summary_to_review,
        }
    }

    pub fn tip_amount(&self) -> Decimal192 {
        let lhs = Decimal192::from(self.tip_percentage as u32)
            / Decimal192::from(100);
        let rhs = self.total_execution_cost
            + self.fee_summary.summary.finalization_cost;
        lhs * rhs
    }

    pub fn total(&self) -> Decimal192 {
        self.fee_summary.total()
            + self.padding_fee
            + self.tip_amount()
            + self.paid_by_dapps
    }
}
