use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
pub struct SignResponse<ID: SignableID> {
    pub per_factor_outcome:
        IndexMap<FactorSourceIDFromHash, PerFactorOutcome<ID>>,
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
    }

    fn sample_other() -> Self {
        Self::failure_with_factors(IndexSet::just(
            FactorSourceIDFromHash::sample_other(),
        ))
    }
}

impl<ID: SignableID> SignResponse<ID> {
    #[allow(unused)]
    pub fn new(
        per_factor_outcome: IndexMap<
            FactorSourceIDFromHash,
            PerFactorOutcome<ID>,
        >,
    ) -> Self {
        Self { per_factor_outcome }
    }

    #[allow(unused)]
    pub fn signed(
        produced_signatures: IndexMap<
            FactorSourceIDFromHash,
            IndexSet<HDSignature<ID>>,
        >,
    ) -> Self {
        Self {
            per_factor_outcome: IndexMap::from_iter(
                produced_signatures.iter().map(|(id, signatures)| {
                    (*id, PerFactorOutcome::signed(*id, signatures.clone()))
                }),
            ),
        }
    }

    #[allow(unused)]
    pub(crate) fn failure_with_factors(
        ids: IndexSet<FactorSourceIDFromHash>,
    ) -> Self {
        Self {
            per_factor_outcome: IndexMap::from_iter(
                ids.iter().map(|id| (*id, PerFactorOutcome::failure(*id))),
            ),
        }
    }

    #[allow(unused)]
    pub(crate) fn user_skipped_factors(
        ids: IndexSet<FactorSourceIDFromHash>,
    ) -> Self {
        Self {
            per_factor_outcome: IndexMap::from_iter(
                ids.iter().map(|id| (*id, PerFactorOutcome::skipped(*id))),
            ),
        }
    }

    #[allow(unused)]
    pub(crate) fn user_skipped_factor(id: FactorSourceIDFromHash) -> Self {
        Self::user_skipped_factors(IndexSet::from_iter([id]))
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
                ids.iter()
                    .map(|id| (*id, PerFactorOutcome::irrelevant(*id))),
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
}
