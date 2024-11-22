use crate::prelude::*;
use sargon::FactorSourceIDFromHash as InternalFactorSourceIDFromHash;
use sargon::NeglectFactorReason as InternalNeglectFactorReason;
type InternalNeglectedFactors = sargon::AbstractNeglectedFactor<sargon::IndexSet<InternalFactorSourceIDFromHash>>;

#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct NeglectedFactors {
    /// The reason why this factor was neglected.
    pub reason: NeglectFactorReason,

    /// The neglected factors
    pub factors: Vec<FactorSourceIDFromHash>,
}

impl NeglectedFactors {

    pub fn into_internal(&self) -> InternalNeglectedFactors {
        self.clone().into()
    }

}

impl From<InternalNeglectedFactors> for NeglectedFactors {
    fn from(value: InternalNeglectedFactors) -> Self {
        Self {
            reason: value.reason.into(),
            factors: value.content.into_iter().map(|id| id.into()).collect(),
        }
    }
}

impl From<NeglectedFactors> for InternalNeglectedFactors {
    fn from(value: NeglectedFactors) -> Self {
        Self::new(
            value.reason.into_internal(),
            value.factors.into_iter().map(|id| id.into_internal()).collect::<sargon::IndexSet::<_>>(),
        )
    }
}


/// Reason why some FactorSource was neglected, either explicitly skipped by the user
/// or implicitly neglected due to failure.
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum
)]
pub enum NeglectFactorReason {
    /// A FactorSource got neglected since user explicitly skipped it.
    UserExplicitlySkipped,

    /// A FactorSource got neglected implicitly due to failure
    Failure,

    /// A FactorSource got neglected implicitly since it is irrelevant,
    /// all transactions which references the FactorSource have already
    /// failed, thus pointless in using it.
    Irrelevant,

    /// We simulate neglect in order to see what the status of petitions
    /// would be if a FactorSource would be neglected.
    Simulation,
}