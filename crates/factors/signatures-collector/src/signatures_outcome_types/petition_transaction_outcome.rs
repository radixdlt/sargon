use crate::prelude::*;

/// The outcome of collecting signatures for a specific
/// transaction - either valid or invalid - and a
/// set of collected signatures (might be empty) and
/// a set of neglected factors (might be empty).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PetitionTransactionOutcome<ID: SignableID> {
    pub signable_id: ID,
    pub(crate) transaction_valid: bool,
    pub signatures: IndexSet<HDSignature<ID>>,
    pub per_entity_neglected_factors:
        IndexMap<AddressOfAccountOrPersona, IndexSet<NeglectedFactor>>,
}

impl<ID: SignableID + HasSampleValues> HasSampleValues
    for PetitionTransactionOutcome<ID>
{
    fn sample() -> Self {
        Self::new(
            false,
            ID::sample(),
            IndexSet::new(),
            IndexMap::kv(
                AddressOfAccountOrPersona::sample_account_mainnet(),
                IndexSet::just(NeglectedFactor::sample()),
            ),
        )
    }

    fn sample_other() -> Self {
        let signature = HDSignature::<ID>::sample_other();
        Self::new(
            true,
            signature.payload_id().clone(),
            IndexSet::just(signature),
            IndexMap::kv(
                AddressOfAccountOrPersona::sample_account_mainnet(),
                IndexSet::just(NeglectedFactor::sample_other()),
            ),
        )
    }
}

impl<ID: SignableID> PetitionTransactionOutcome<ID> {
    pub fn neglected_factors(&self) -> IndexSet<NeglectedFactor> {
        self.per_entity_neglected_factors
            .values()
            .flat_map(|neglected_factors| neglected_factors.iter().cloned())
            .collect()
    }

    /// # Panics
    /// Panics if the intent hash in any signatures does not
    /// match `intent_hash`
    pub(crate) fn new(
        transaction_valid: bool,
        signable_id: ID,
        signatures: IndexSet<HDSignature<ID>>,
        per_entity_neglected_factors: IndexMap<
            AddressOfAccountOrPersona,
            IndexSet<NeglectedFactor>,
        >,
    ) -> Self {
        assert!(
            signatures.iter().all(|s| *s.payload_id() == signable_id),
            "Discrepancy! Mismatching intent hash found in a signature."
        );
        Self {
            signable_id,
            transaction_valid,
            signatures,
            per_entity_neglected_factors,
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
            IndexMap::new(),
        );
    }
}
