use crate::prelude::*;

/// Represents a summary of fees to review.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct FeeSummaryToReview {
    pub summary: FeeSummary,
    pub guarantees_cost: Decimal192,
    pub lock_fee_cost: Decimal192,
    pub signatures_cost: Decimal192,
    pub notarizing_cost: Decimal192,
    pub non_contingent_lock: Decimal192,
    pub contingent_lock: Decimal192,
}

impl FeeSummaryToReview {
    /// Creates a new `FeeSummaryToReview` from an `ExecutionSummary`.
    pub fn new_from_execution_summary(
        execution_summary: ExecutionSummary,
        signatures_count: usize,
        notary_is_signatory: bool,
        include_lock_fee: bool,
    ) -> Self {
        Self {
            summary: execution_summary.fee_summary,
            guarantees_cost: Self::calculate_guarantees_cost(
                &execution_summary.detailed_classification,
                &execution_summary.deposits,
            ),
            lock_fee_cost: FeeConstants::lock_fee_cost(include_lock_fee),
            signatures_cost: FeeConstants::signatures_cost(signatures_count),
            notarizing_cost: FeeConstants::notarizing_cost(notary_is_signatory),
            non_contingent_lock: execution_summary.fee_locks.lock,
            contingent_lock: execution_summary.fee_locks.contingent_lock,
        }
    }

    /// Calculates the total execution cost.
    pub fn total_execution_cost(&self) -> Decimal192 {
        self.summary.execution_cost
            + self.guarantees_cost
            + self.signatures_cost
            + self.lock_fee_cost
            + self.notarizing_cost
    }

    /// Calculates the total cost.
    pub fn total(&self) -> Decimal192 {
        self.total_execution_cost()
            + self.summary.finalization_cost
            + self.summary.storage_expansion_cost
            + self.summary.royalty_cost
    }

    /// Calculates the guarantees cost based on transfer type and deposits.
    fn calculate_guarantees_cost(
        detailed_classification: &[DetailedManifestClass],
        deposits: &HashMap<AccountAddress, Vec<ResourceIndicator>>,
    ) -> Decimal192 {
        match detailed_classification.first() {
            Some(DetailedManifestClass::General)
            | Some(DetailedManifestClass::Transfer { is_one_to_one: _ }) => {
                deposits.iter().flat_map(|deposit| deposit.1).fold(
                    Decimal192::zero(),
                    |result, resource| match resource {
                        ResourceIndicator::Fungible {
                            resource_address: _,
                            indicator: _,
                        } => result
                            + FeeConstants::fungible_guarantee_instruction_cost(
                            ),
                        _ => result,
                    },
                )
            }
            _ => Decimal192::zero(),
        }
    }
}
