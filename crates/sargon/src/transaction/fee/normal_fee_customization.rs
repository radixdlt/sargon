use crate::prelude::*;

/// Represents the customization for normal fee mode.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct NormalFeeCustomization {
    pub network_fee: Decimal192,
    pub royalty_fee: Decimal192,
    pub total: Decimal192,
}

impl NormalFeeCustomization {
    /// Creates a new `NormalFeeCustomization` with the given network and royalty fees.
    pub fn new(network_fee: Decimal192, royalty_fee: Decimal192) -> Self {
        Self {
            network_fee,
            royalty_fee,
            total: network_fee + royalty_fee,
        }
    }

    /// Creates a new `NormalFeeCustomization` from a `FeeSummaryToReview` and `FeeLocks`.
    pub fn new_from_summary(
        fee_summary_to_review: FeeSummaryToReview,
        fee_locks: FeeLocks,
    ) -> Self {
        let network_fee = (fee_summary_to_review.total_execution_cost()
            + fee_summary_to_review.summary.finalization_cost
            + fee_summary_to_review.summary.storage_expansion_cost)
            * (Decimal192::one() + FeeConstants::network_fee_multiplier());
        let remaining_non_contingent_lock =
            (fee_locks.lock - network_fee).clamped_to_zero();

        Self::new(
            (network_fee - fee_locks.lock).clamped_to_zero(),
            (fee_summary_to_review.summary.royalty_cost
                - remaining_non_contingent_lock)
                .clamped_to_zero(),
        )
    }
}
