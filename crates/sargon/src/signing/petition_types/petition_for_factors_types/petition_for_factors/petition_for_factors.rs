use super::*;
use crate::prelude::*;

/// Petition of signatures from a factors list of an entity in a transaction.
#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
#[debug("{}", self.debug_str())]
pub struct PetitionForFactors<ID: SignableID> {
    pub factor_list_kind: FactorListKind,

    /// Factors to sign with and the required number of them.
    pub input: PetitionForFactorsInput,
    state: RefCell<PetitionForFactorsState<ID>>,
}

impl<ID: SignableID> HasSampleValues for PetitionForFactors<ID> {
    fn sample() -> Self {
        Self::new(FactorListKind::Threshold, PetitionForFactorsInput::sample())
    }

    fn sample_other() -> Self {
        Self::new(
            FactorListKind::Override,
            PetitionForFactorsInput::sample_other(),
        )
    }
}

impl<ID: SignableID> PetitionForFactors<ID> {
    pub fn new(
        factor_list_kind: FactorListKind,
        input: PetitionForFactorsInput,
    ) -> Self {
        Self {
            factor_list_kind,
            input,
            state: RefCell::new(PetitionForFactorsState::new()),
        }
    }

    pub fn factor_instances(
        &self,
    ) -> IndexSet<HierarchicalDeterministicFactorInstance> {
        self.input.factors.clone()
    }

    pub fn all_neglected(&self) -> IndexSet<NeglectedFactorInstance> {
        self.state.borrow().all_neglected()
    }

    pub fn all_signatures(&self) -> IndexSet<HDSignature<ID>> {
        self.state.borrow().all_signatures()
    }

    pub fn new_threshold(
        factors: Vec<HierarchicalDeterministicFactorInstance>,
        threshold: i8,
    ) -> Option<Self> {
        if factors.is_empty() {
            return None;
        }
        Some(Self::new(
            FactorListKind::Threshold,
            PetitionForFactorsInput::new_threshold(
                IndexSet::from_iter(factors),
                threshold,
            ),
        ))
    }

    pub fn new_unsecurified(
        factor: HierarchicalDeterministicFactorInstance,
    ) -> Self {
        Self::new_threshold(vec![factor], 1).expect("Factors is not empty") // define as 1/1 threshold factor, which is a good definition.
    }

    pub fn new_override(
        factors: Vec<HierarchicalDeterministicFactorInstance>,
    ) -> Option<Self> {
        if factors.is_empty() {
            return None;
        }
        Some(Self::new(
            FactorListKind::Override,
            PetitionForFactorsInput::new_override(IndexSet::from_iter(factors)),
        ))
    }

    pub fn neglect_if_referenced(&self, neglected: NeglectedFactor) {
        let factor_source_id = &neglected.factor_source_id();
        if let Some(_x_) =
            self.reference_to_factor_source_with_id(factor_source_id)
        {
            debug!(
                "PetitionForFactors = kind {:?} neglect factor source with id: {}, reason: {}",
                self.factor_list_kind, factor_source_id, neglected.reason
            );
            self.neglect(neglected)
        } else {
            debug!(
                "PetitionForFactors = kind {:?} did not reference factor source with id: {}",
                self.factor_list_kind, factor_source_id
            );
        }
    }

    fn neglect(&self, neglected: NeglectedFactor) {
        let factor_instance = self.expect_reference_to_factor_source_with_id(
            &neglected.factor_source_id(),
        );
        self.state
            .borrow_mut()
            .neglect(&NeglectedFactorInstance::new(
                neglected.reason,
                factor_instance.clone(),
            ));
    }

    pub fn has_owned_instance_with_id(
        &self,
        owned_factor_instance: &OwnedFactorInstance,
    ) -> bool {
        self.has_instance_with_id(owned_factor_instance.factor_instance())
    }

    pub fn has_instance_with_id(
        &self,
        factor_instance: &HierarchicalDeterministicFactorInstance,
    ) -> bool {
        self.input.factors.iter().any(|f| f == factor_instance)
    }

    pub fn add_signature_if_relevant(
        &self,
        signature: &HDSignature<ID>,
    ) -> bool {
        if self.has_owned_instance_with_id(signature.owned_factor_instance()) {
            self.add_signature(signature);
            true
        } else {
            false
        }
    }

    /// # Panics
    /// Panics if this factor source has already been neglected or signed with.
    fn add_signature(&self, signature: &HDSignature<ID>) {
        let state = self.state.borrow_mut();
        state.add_signature(signature)
    }

    pub fn references_factor_source_with_id(
        &self,
        factor_source_id: &FactorSourceIDFromHash,
    ) -> bool {
        self.reference_to_factor_source_with_id(factor_source_id)
            .is_some()
    }

    fn expect_reference_to_factor_source_with_id(
        &self,
        factor_source_id: &FactorSourceIDFromHash,
    ) -> &HierarchicalDeterministicFactorInstance {
        self.reference_to_factor_source_with_id(factor_source_id)
            .expect("Programmer error! Factor source not found in factors.")
    }

    fn reference_to_factor_source_with_id(
        &self,
        factor_source_id: &FactorSourceIDFromHash,
    ) -> Option<&HierarchicalDeterministicFactorInstance> {
        self.input.reference_factor_source_with_id(factor_source_id)
    }

    fn state_snapshot(&self) -> PetitionForFactorsStateSnapshot<ID> {
        self.state.borrow().snapshot()
    }

    fn is_finished_successfully(&self) -> bool {
        self.input.is_fulfilled_by(self.state_snapshot())
    }

    fn is_finished_with_fail(&self) -> bool {
        let snapshot = self.state_snapshot();
        let is_finished_with_fail =
            self.input.is_failure_with(snapshot.clone());
        trace!(
            "is_finished_with_fail: {:?} from input: {:?}, snapshot: {:?}",
            is_finished_with_fail,
            self.input,
            snapshot
        );
        is_finished_with_fail
    }

    fn get_finished_with(&self) -> Option<PetitionFactorsStatusFinished> {
        if self.is_finished_successfully() {
            Some(PetitionFactorsStatusFinished::Success)
        } else if self.is_finished_with_fail() {
            Some(PetitionFactorsStatusFinished::Fail)
        } else {
            None
        }
    }

    pub fn status(&self) -> PetitionForFactorsStatus {
        if let Some(finished_state) = self.get_finished_with() {
            return PetitionForFactorsStatus::Finished(finished_state);
        }
        PetitionForFactorsStatus::InProgress
    }

    #[allow(unused)]
    pub fn debug_str(&self) -> String {
        format!(
            "PetitionForFactors(input: {:#?}, state_snapshot: {:#?})",
            self.input,
            self.state_snapshot()
        )
    }
}
