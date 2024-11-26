use crate::prelude::*;
use sargon::TransactionIntentHash as InternalTransactionIntentHash;

type InternalSignWithFactorsOutcomeForTransactionIntent =
    sargon::SignWithFactorsOutcome<InternalTransactionIntentHash>;

#[derive(Clone, PartialEq, Eq, uniffi::Enum)]
pub enum SignWithFactorsOutcomeForTransactionIntent {
    /// The user successfully signed with the factor source(s), the associated
    /// value contains the produces signatures and any relevant metadata.
    Signed {
        produced_signatures: SignResponseForTransactionIntent,
    },

    /// The factor source got neglected, either due to user explicitly skipping
    /// or due to failure
    Neglected(NeglectedFactors),
}

impl SignWithFactorsOutcomeForTransactionIntent {
    pub fn into_internal(
        &self,
    ) -> InternalSignWithFactorsOutcomeForTransactionIntent {
        self.clone().into()
    }
}

impl From<InternalSignWithFactorsOutcomeForTransactionIntent>
    for SignWithFactorsOutcomeForTransactionIntent
{
    fn from(value: InternalSignWithFactorsOutcomeForTransactionIntent) -> Self {
        match value {
            InternalSignWithFactorsOutcomeForTransactionIntent::Signed {
                produced_signatures,
            } => SignWithFactorsOutcomeForTransactionIntent::Signed {
                produced_signatures: produced_signatures.into(),
            },
            InternalSignWithFactorsOutcomeForTransactionIntent::Neglected(
                factors,
            ) => SignWithFactorsOutcomeForTransactionIntent::Neglected(
                factors.into(),
            ),
        }
    }
}

impl From<SignWithFactorsOutcomeForTransactionIntent>
    for InternalSignWithFactorsOutcomeForTransactionIntent
{
    fn from(value: SignWithFactorsOutcomeForTransactionIntent) -> Self {
        match value {
            SignWithFactorsOutcomeForTransactionIntent::Signed {
                produced_signatures,
            } => Self::Signed {
                produced_signatures: produced_signatures.into_internal(),
            },
            SignWithFactorsOutcomeForTransactionIntent::Neglected(factors) => {
                Self::Neglected(factors.into_internal())
            }
        }
    }
}
