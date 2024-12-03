use crate::prelude::*;
use sargon::TransactionIntentHash as InternalTransactionIntentHash;
use crate::signing::sign_response::SignResponseOfTransactionIntentHash;

type InternalSignWithFactorsOutcomeForTransactionIntent =
    sargon::SignWithFactorsOutcome<InternalTransactionIntentHash>;

#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum SignWithFactorsOutcomeForTransactionIntent {
    /// The user successfully signed with the factor source(s), the associated
    /// value contains the produces signatures and any relevant metadata.
    Signed {
        produced_signatures: SignResponseOfTransactionIntentHash,
    },

    /// The factor source got neglected, either due to user explicitly skipping
    /// or due to failure
    Neglected(NeglectedFactors),
}
