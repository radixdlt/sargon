use crate::prelude::*;

/// The outcome of collecting signatures for a specific
/// transasction - either valid or invalid - and a
/// set of collected signatues (might be empty) and
/// a set of neglected factors (might be empty).
#[derive(Clone, PartialEq, Eq)]
pub(crate) struct PetitionTransactionOutcome<ID: SignablePayloadID> {
    payload_id: ID,
    pub(crate) transaction_valid: bool,
    pub(crate) signatures: IndexSet<HDSignature<ID>>,
    pub(crate) neglected_factors: IndexSet<NeglectedFactor>,
}

impl <ID: SignablePayloadID> PetitionTransactionOutcome<ID> {
    /// # Panics
    /// Panics if the intent hash in any signatures does not
    /// match `intent_hash`
    pub(crate) fn new(
        transaction_valid: bool,
        payload_id: ID,
        signatures: IndexSet<HDSignature<ID>>,
        neglected_factors: IndexSet<NeglectedFactor>,
    ) -> Self {
        assert!(
            signatures.iter().all(|s| *s.payload_id() == payload_id),
            "Discprenacy! Mismatching intent hash found in a signature."
        );
        Self {
            payload_id,
            transaction_valid,
            signatures,
            neglected_factors,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = PetitionTransactionOutcome<TransactionIntentHash>;

    #[test]
    #[should_panic(
        expected = "Discprenacy! Mismatching intent hash found in a signature."
    )]
    fn panic() {
        Sut::new(
            true,
            TransactionIntentHash::sample(),
            IndexSet::just(HDSignature::sample_other()),
            IndexSet::new(),
        );
    }
}
