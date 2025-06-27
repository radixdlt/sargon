use crate::prelude::*;

/// The outcome of collecting signatures for a specific
/// transaction - either valid or invalid - and a
/// set of collected signatures (might be empty) and
/// a set of neglected factors (might be empty).
#[derive(Clone, PartialEq, Eq)]
pub(crate) struct PetitionTransactionOutcome<ID: SignableID> {
    signable_id: ID,
    pub(crate) transaction_valid: bool,
    pub(crate) signatures: IndexSet<HDSignature<ID>>,
    pub(crate) neglected_factors: IndexSet<NeglectedFactor>,
}

impl<ID: SignableID> PetitionTransactionOutcome<ID> {
    /// # Panics
    /// Panics if the intent hash in any signatures does not
    /// match `intent_hash`
    pub(crate) fn new(
        transaction_valid: bool,
        signable_id: ID,
        signatures: IndexSet<HDSignature<ID>>,
        neglected_factors: IndexSet<NeglectedFactor>,
    ) -> Self {
        assert!(
            signatures.iter().all(|s| *s.payload_id() == signable_id),
            "Discrepancy! Mismatching intent hash found in a signature."
        );
        Self {
            signable_id,
            transaction_valid,
            signatures,
            neglected_factors,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PetitionTransactionOutcome<TransactionIntentHash>;

    #[test]
    #[should_panic(
        expected = "Discrepancy! Mismatching intent hash found in a signature."
    )]
    fn panic() {
        SUT::new(
            true,
            TransactionIntentHash::sample(),
            IndexSet::just(HDSignature::sample_other()),
            IndexSet::new(),
        );
    }
}
