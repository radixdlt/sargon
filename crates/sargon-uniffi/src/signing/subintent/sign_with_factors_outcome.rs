use crate::prelude::*;
use sargon::SignWithFactorsOutcome as InternalSignWithFactorsOutcome;
use sargon::SubintentHash as InternalSubintentHash;

type InternalSignWithFactorsOutcomeForSubintent =
    InternalSignWithFactorsOutcome<InternalSubintentHash>;

#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum SignWithFactorsOutcomeForSubintent {
    /// The user successfully signed with the factor source(s), the associated
    /// value contains the produces signatures and any relevant metadata.
    Signed {
        produced_signatures: SignResponseForSubintent,
    },

    /// The factor source got neglected, either due to user explicitly skipping
    /// or due to failure
    Neglected(NeglectedFactors),
}
