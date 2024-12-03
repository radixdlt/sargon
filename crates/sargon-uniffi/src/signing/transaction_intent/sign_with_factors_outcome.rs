use crate::prelude::*;
use sargon::TransactionIntentHash as InternalTransactionIntentHash;

type InternalSignWithFactorsOutcomeForTransactionIntent =
    sargon::SignWithFactorsOutcome<InternalTransactionIntentHash>;

#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
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
