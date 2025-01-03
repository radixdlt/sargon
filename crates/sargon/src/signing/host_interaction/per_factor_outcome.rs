use crate::prelude::*;

/// The outcome of the signing process for each factor source as collected by the `SignInteractor`.
#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
pub enum PerFactorOutcome<ID: SignableID> {
    /// The user successfully signed with the factor source, the associated
    /// value contains the produced signatures and any relevant metadata.
    #[debug("Signed: {:#?}", produced_signatures)]
    Signed {
        factor_source_id: FactorSourceIDFromHash,
        produced_signatures: IndexSet<HDSignature<ID>>,
    },

    /// The factor source got neglected, either due to user explicitly skipping
    /// or due to failure
    #[debug("Neglected")]
    Neglected(NeglectedFactor),
}

impl<ID: SignableID> PerFactorOutcome<ID> {
    pub fn signed(
        factor_source_id: FactorSourceIDFromHash,
        produced_signatures: IndexSet<HDSignature<ID>>,
    ) -> Self {
        PerFactorOutcome::Signed {
            factor_source_id,
            produced_signatures,
        }
    }

    pub fn failure(factor: FactorSourceIDFromHash) -> Self {
        PerFactorOutcome::Neglected(NeglectedFactor::new(
            NeglectFactorReason::Failure,
            factor,
        ))
    }

    pub fn skipped(factor: FactorSourceIDFromHash) -> Self {
        PerFactorOutcome::Neglected(NeglectedFactor::new(
            NeglectFactorReason::UserExplicitlySkipped,
            factor,
        ))
    }

    pub fn irrelevant(factor: FactorSourceIDFromHash) -> Self {
        PerFactorOutcome::Neglected(NeglectedFactor::new(
            NeglectFactorReason::Irrelevant,
            factor,
        ))
    }

    pub fn factor_source_id(&self) -> FactorSourceIDFromHash {
        match self {
            PerFactorOutcome::Signed {
                factor_source_id, ..
            } => *factor_source_id,
            PerFactorOutcome::Neglected(neglected_factor) => {
                neglected_factor.content
            }
        }
    }
}
