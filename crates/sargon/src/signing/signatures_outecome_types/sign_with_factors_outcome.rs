use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
pub enum SignWithFactorsOutcome<ID: SignableID> {
    /// The user successfully signed with the factor source(s), the associated
    /// value contains the produces signatures and any relevant metadata.
    #[debug("Signed: {:#?}", produced_signatures)]
    Signed {
        produced_signatures: SignResponse<ID>,
    },

    /// The factor source got neglected, either due to user explicitly skipping
    /// or due to failure
    #[debug("Neglected")]
    Neglected(NeglectedFactors),
}

impl<ID: SignableID> HasSampleValues for SignWithFactorsOutcome<ID> {
    fn sample() -> Self {
        Self::signed(SignResponse::sample())
    }

    fn sample_other() -> Self {
        Self::user_skipped_factor(FactorSourceIDFromHash::sample_other())
    }
}

impl<ID: SignableID> SignWithFactorsOutcome<ID> {
    #[allow(unused)]
    pub fn signed(produced_signatures: SignResponse<ID>) -> Self {
        Self::Signed {
            produced_signatures,
        }
    }

    #[allow(unused)]
    pub(crate) fn failure_with_factors(
        ids: IndexSet<FactorSourceIDFromHash>,
    ) -> Self {
        Self::Neglected(NeglectedFactors::new(
            NeglectFactorReason::Failure,
            ids,
        ))
    }

    #[allow(unused)]
    pub(crate) fn user_skipped_factors(
        ids: IndexSet<FactorSourceIDFromHash>,
    ) -> Self {
        Self::Neglected(NeglectedFactors::new(
            NeglectFactorReason::UserExplicitlySkipped,
            ids,
        ))
    }

    #[allow(unused)]
    pub(crate) fn user_skipped_factor(id: FactorSourceIDFromHash) -> Self {
        Self::user_skipped_factors(IndexSet::from_iter([id]))
    }

    pub(crate) fn irrelevant(
        factor_sources_of_kind: &FactorSourcesOfKind,
    ) -> Self {
        Self::Neglected(NeglectedFactors::new(
            NeglectFactorReason::Irrelevant,
            factor_sources_of_kind
                .factor_sources()
                .into_iter()
                .map(|f| *f.factor_source_id().as_hash().unwrap()) // TODO ask that
                .collect(),
        ))
    }
}
