use crate::prelude::*;

/// A neglected factor, with a reason.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AbstractNeglectedFactor<T> {
    /// The reason why this factor was neglected.
    pub(crate) reason: NeglectFactorReason,

    /// The neglected factor
    pub(crate) content: T,
}

impl<T> AbstractNeglectedFactor<T> {
    pub fn new(reason: NeglectFactorReason, content: T) -> Self {
        Self { reason, content }
    }
}

impl<T: Debug> Debug for AbstractNeglectedFactor<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Neglected")
            .field("reason", &self.reason)
            .field("content", &self.content)
            .finish()
    }
}

impl NeglectedFactorInstance {
    /// Maps from `Neglected<HierarchicalDeterministicFactorInstance>`
    /// to `Neglected<FactorSourceIDFromHash>`,
    pub(crate) fn as_neglected_factor(&self) -> NeglectedFactor {
        NeglectedFactor::new(self.reason, self.factor_source_id())
    }
}
impl FactorSourceReferencing for NeglectedFactorInstance {
    fn factor_source_id(&self) -> FactorSourceIDFromHash {
        self.content.factor_source_id()
    }
}

impl FactorSourceReferencing for NeglectedFactor {
    fn factor_source_id(&self) -> FactorSourceIDFromHash {
        self.content
    }
}

impl HasSampleValues for NeglectedFactorInstance {
    fn sample() -> Self {
        Self::new(
            NeglectFactorReason::UserExplicitlySkipped,
            HierarchicalDeterministicFactorInstance::sample(),
        )
    }
    fn sample_other() -> Self {
        Self::new(
            NeglectFactorReason::Failure,
            HierarchicalDeterministicFactorInstance::sample_other(),
        )
    }
}

/// ID to some neglected factor source, with the reason why it was neglected (skipped/failed)
pub(crate) type NeglectedFactor =
    AbstractNeglectedFactor<FactorSourceIDFromHash>;

/// IDs to some neglected factor source, with the reason why they were neglected (skipped/failed)
pub type NeglectedFactors =
    AbstractNeglectedFactor<IndexSet<FactorSourceIDFromHash>>;

/// A HierarchicalDeterministicFactorInstance which was rejected, with the reason why (skipped/failed)
pub(crate) type NeglectedFactorInstance =
    AbstractNeglectedFactor<HierarchicalDeterministicFactorInstance>;

/// Reason why some FactorSource was neglected, either explicitly skipped by the user
/// or implicitly neglected due to failure.
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, derive_more::Debug, derive_more::Display,
)]
pub enum NeglectFactorReason {
    /// A FactorSource got neglected since user explicitly skipped it.
    #[display("User Skipped")]
    #[debug("UserExplicitlySkipped")]
    UserExplicitlySkipped,

    /// A FactorSource got neglected implicitly due to failure
    #[display("Failure")]
    #[debug("Failure")]
    Failure,

    /// A FactorSource got neglected implicitly since it is irrelevant,
    /// all transactions which references the FactorSource have already
    /// failed, thus pointless in using it.
    #[display("Irrelevant")]
    #[debug("Irrelevant")]
    Irrelevant,

    /// We simulate neglect in order to see what the status of petitions
    /// would be if a FactorSource would be neglected.
    #[display("Simulation")]
    #[debug("Simulation")]
    Simulation,
}
