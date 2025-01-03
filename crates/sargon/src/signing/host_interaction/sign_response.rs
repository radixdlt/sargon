use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
pub struct SignResponse<ID: SignableID> {
    pub per_factor_outcome: IndexMap<FactorSourceIDFromHash, FactorOutcome<ID>>,
}

impl<ID: SignableID + HasSampleValues> HasSampleValues for SignResponse<ID> {
    fn sample() -> Self {
        let hd_signature = HDSignature::sample();
        let factor_source_id = hd_signature
            .input
            .owned_factor_instance
            .value
            .factor_source_id;

        Self::signed(IndexMap::just((
            factor_source_id,
            IndexSet::just(hd_signature),
        )))
        .unwrap()
    }

    fn sample_other() -> Self {
        Self::user_skipped_factors(IndexSet::just(
            FactorSourceIDFromHash::sample_other(),
        ))
    }
}

impl<ID: SignableID> SignResponse<ID> {
    #[allow(unused)]
    pub fn new_from_outcomes(
        outcomes: IndexMap<FactorSourceIDFromHash, FactorOutcome<ID>>,
    ) -> Result<Self> {
        if outcomes
            .iter()
            .any(|(id, outcome)| *id != outcome.factor_source_id())
        {
            return Err(CommonError::FactorOutcomeSignedFactorSourceIDMismatch);
        }

        Ok(Self {
            per_factor_outcome: outcomes,
        })
    }

    #[allow(unused)]
    pub fn signed(
        produced_signatures: IndexMap<
            FactorSourceIDFromHash,
            IndexSet<HDSignature<ID>>,
        >,
    ) -> Result<Self> {
        let signed_outcomes = produced_signatures
            .iter()
            .map(|(id, signatures)| {
                let outcome = FactorOutcome::signed(signatures.clone())?;
                Ok((*id, outcome))
            })
            .collect::<Result<Vec<(FactorSourceIDFromHash, FactorOutcome<ID>)>>>()?;

        Ok(Self {
            per_factor_outcome: IndexMap::from_iter(signed_outcomes),
        })
    }

    #[allow(unused)]
    pub(crate) fn failure_with_factors(
        ids: IndexSet<FactorSourceIDFromHash>,
    ) -> Self {
        Self {
            per_factor_outcome: IndexMap::from_iter(
                ids.iter().map(|id| (*id, FactorOutcome::failure(*id))),
            ),
        }
    }

    pub fn user_skipped_factors(ids: IndexSet<FactorSourceIDFromHash>) -> Self {
        Self {
            per_factor_outcome: IndexMap::from_iter(
                ids.iter().map(|id| (*id, FactorOutcome::skipped(*id))),
            ),
        }
    }

    pub(crate) fn irrelevant(
        factor_sources_of_kind: &FactorSourcesOfKind,
    ) -> Self {
        let ids = factor_sources_of_kind
            .factor_sources()
            .into_iter()
            .map(|f| *f.factor_source_id().as_hash().unwrap()) // TODO ask that
            .collect_vec();

        Self {
            per_factor_outcome: IndexMap::from_iter(
                ids.iter().map(|id| (*id, FactorOutcome::irrelevant(*id))),
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SignResponse<TransactionIntentHash>;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn test_new_from_outcomes_fails_due_to_id_mismatch() {
        let outcome = FactorOutcome::sample();

        let wrong_outcome_paring = IndexMap::from([
            (outcome.factor_source_id(), outcome.clone()),
            (FactorSourceIDFromHash::sample_ledger(), outcome),
        ]);
        let result = SUT::new_from_outcomes(wrong_outcome_paring);

        assert_eq!(
            Err(CommonError::FactorOutcomeSignedFactorSourceIDMismatch),
            result
        );
    }
}
