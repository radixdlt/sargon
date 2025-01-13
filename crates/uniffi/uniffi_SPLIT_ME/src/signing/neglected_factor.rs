use crate::prelude::*;
use sargon::FactorSourceIDFromHash as InternalFactorSourceIDFromHash;
use sargon::NeglectFactorReason as InternalNeglectFactorReason;
type InternalNeglectedFactor =
    sargon::AbstractNeglectedFactor<InternalFactorSourceIDFromHash>;

#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct NeglectedFactor {
    /// The reason why this factor was neglected.
    pub reason: NeglectFactorReason,

    /// The neglected factors
    pub factor: FactorSourceIDFromHash,
}

impl NeglectedFactor {
    pub fn into_internal(&self) -> InternalNeglectedFactor {
        self.clone().into()
    }
}

impl From<InternalNeglectedFactor> for NeglectedFactor {
    fn from(value: InternalNeglectedFactor) -> Self {
        Self {
            reason: value.reason.into(),
            factor: value.content.into(),
        }
    }
}

impl From<NeglectedFactor> for InternalNeglectedFactor {
    fn from(value: NeglectedFactor) -> Self {
        Self::new(value.reason.into_internal(), value.factor.into_internal())
    }
}

/// Reason why some FactorSource was neglected, either explicitly skipped by the user
/// or implicitly neglected due to failure.
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum,
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
