use crate::prelude::*;
use sargon::SignWithFactorsOutcome as InternalSignWithFactorsOutcome;
use sargon::SubintentHash as InternalSubintentHash;

type InternalSignWithFactorsOutcomeForSubintent =
    InternalSignWithFactorsOutcome<InternalSubintentHash>;

#[derive(Clone, PartialEq, Eq, uniffi::Enum)]
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

impl SignWithFactorsOutcomeForSubintent {
    pub fn into_internal(&self) -> InternalSignWithFactorsOutcomeForSubintent {
        self.clone().into()
    }
}

impl From<InternalSignWithFactorsOutcomeForSubintent>
    for SignWithFactorsOutcomeForSubintent
{
    fn from(value: InternalSignWithFactorsOutcomeForSubintent) -> Self {
        match value {
            InternalSignWithFactorsOutcomeForSubintent::Signed {
                produced_signatures,
            } => SignWithFactorsOutcomeForSubintent::Signed {
                produced_signatures: produced_signatures.into(),
            },
            InternalSignWithFactorsOutcomeForSubintent::Neglected(factors) => {
                SignWithFactorsOutcomeForSubintent::Neglected(factors.into())
            }
        }
    }
}

impl From<SignWithFactorsOutcomeForSubintent>
    for InternalSignWithFactorsOutcomeForSubintent
{
    fn from(value: SignWithFactorsOutcomeForSubintent) -> Self {
        match value {
            SignWithFactorsOutcomeForSubintent::Signed {
                produced_signatures,
            } => Self::Signed {
                produced_signatures: produced_signatures.into_internal(),
            },
            SignWithFactorsOutcomeForSubintent::Neglected(factors) => {
                Self::Neglected(factors.into_internal())
            }
        }
    }
}

decl_conversion_tests_for!(SignWithFactorsOutcomeForSubintent);
