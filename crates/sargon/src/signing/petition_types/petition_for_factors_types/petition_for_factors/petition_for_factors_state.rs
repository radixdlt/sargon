use std::cell::Ref;

use super::*;
use crate::prelude::*;

/// Mutable state of `PetitionForFactors`, keeping track of which factors that
/// have either signed or been neglected.
#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
#[debug("PetitionForFactorsState(signed: {:?}, neglected: {:?})", signed.borrow().clone(), neglected.borrow().clone())]
pub(crate) struct PetitionForFactorsState {
    /// Factors that have signed.
    signed: RefCell<PetitionForFactorsSubState<HDSignature>>,

    /// Neglected factors, either due to user explicitly skipping, or due
    /// implicitly neglected to failure.
    neglected: RefCell<PetitionForFactorsSubState<NeglectedFactorInstance>>,
}

impl PetitionForFactorsState {
    /// Creates a new `PetitionForFactorsState`.
    pub(super) fn new() -> Self {
        Self {
            signed: RefCell::new(PetitionForFactorsSubState::<_>::new()),
            neglected: RefCell::new(PetitionForFactorsSubState::<_>::new()),
        }
    }

    /// A reference to the neglected factors so far.
    pub(super) fn neglected(&self) -> Ref<PetitionForFactorsSubState<NeglectedFactorInstance>> {
        self.neglected.borrow()
    }

    /// A reference to the factors which have been signed with so far.
    pub(super) fn signed(&self) -> Ref<PetitionForFactorsSubState<HDSignature>> {
        self.signed.borrow()
    }

    /// A set of signatures from factors that have been signed with so far.
    pub(crate) fn all_signatures(&self) -> IndexSet<HDSignature> {
        self.signed().snapshot()
    }

    /// A set factors have been neglected so far.
    pub(crate) fn all_neglected(&self) -> IndexSet<NeglectedFactorInstance> {
        self.neglected().snapshot()
    }

    /// # Panics
    /// Panics if this factor source has already been neglected or signed with.
    fn assert_not_referencing_factor_source(&self, factor_source_id: FactorSourceIDFromHash) {
        assert!(
            !self.references_factor_source_by_id(factor_source_id),
            "Programmer error! Factor source {:#?} already used, should only be referenced once.",
            factor_source_id,
        );
    }

    /// # Panics
    /// Panics if this factor source has already been neglected or signed and
    /// this is not a simulation.
    pub(crate) fn neglect(&self, neglected: &NeglectedFactorInstance) {
        if neglected.reason != NeglectFactorReason::Simulation {
            self.assert_not_referencing_factor_source(neglected.factor_source_id());
        }
        self.neglected.borrow_mut().insert(neglected);
    }

    /// # Panics
    /// Panics if this factor source has already been neglected or signed with.
    pub(crate) fn add_signature(&self, signature: &HDSignature) {
        self.assert_not_referencing_factor_source(signature.factor_source_id());
        self.signed.borrow_mut().insert(signature)
    }

    pub(super) fn snapshot(&self) -> PetitionForFactorsStateSnapshot {
        PetitionForFactorsStateSnapshot::new(self.signed().snapshot(), self.neglected().snapshot())
    }

    fn references_factor_source_by_id(&self, factor_source_id: FactorSourceIDFromHash) -> bool {
        self.signed()
            .references_factor_source_by_id(factor_source_id)
            || self
            .neglected()
            .references_factor_source_by_id(factor_source_id)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    type Sut = PetitionForFactorsState;

    impl PetitionForFactorsState {
        fn test_neglect(&self, id: &HierarchicalDeterministicFactorInstance, simulated: bool) {
            self.neglect(&NeglectedFactorInstance::new(
                if simulated {
                    NeglectFactorReason::Simulation
                } else {
                    NeglectFactorReason::UserExplicitlySkipped
                },
                id.clone(),
            ))
        }
    }

    #[test]
    #[should_panic]
    fn skipping_twice_panics() {
        let sut = Sut::new();
        let fi = HierarchicalDeterministicFactorInstance::sample();
        sut.test_neglect(&fi, false);
        sut.test_neglect(&fi, false);
    }

    #[test]
    #[should_panic]
    fn signing_twice_panics() {
        let sut = Sut::new();
        let sig = HDSignature::sample();
        sut.add_signature(&sig);
        sut.add_signature(&sig);
    }

    #[test]
    #[should_panic]
    fn skipping_already_signed_panics() {
        let sut = Sut::new();

        let intent_hash = IntentHash::sample();

        let hd_factor_source_id = HDFactorSourceIdFromHash::sample_at(0);

        let hd_signature = hd_factor_source_id.hd_signature(
            intent_hash,
            HDPathComponent::from(0)
        );

        sut.add_signature(&hd_signature);

        sut.test_neglect(&hd_factor_source_id.hd_factor_instance(0), false);
    }

    #[test]
    #[should_panic]
    fn signing_already_skipped_panics() {
        let sut = Sut::new();
        let hd_factor_source_id = HDFactorSourceIdFromHash::sample_at(0);

        sut.test_neglect(&hd_factor_source_id.hd_factor_instance(0), false);

        let intent_hash = IntentHash::sample();
        let hd_signature = hd_factor_source_id.hd_signature(
            intent_hash,
            HDPathComponent::from(0)
        );

        sut.add_signature(&hd_signature);
    }
}
