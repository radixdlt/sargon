use super::*;
use crate::prelude::*;

/// The input passed to a PetitionsForFactors
#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
#[debug("PetitionForFactorsInput(factors: {:#?})", self.factors)]
pub(crate) struct PetitionForFactorsInput {
    /// Factors to sign with.
    pub(super) factors: IndexSet<HierarchicalDeterministicFactorInstance>,

    /// Number of required factors to sign with.
    pub(super) required: i8,
}

impl HasSampleValues for PetitionForFactorsInput {
    fn sample() -> Self {
        Self::new(
            IndexSet::from_iter([
                HierarchicalDeterministicFactorInstance::sample(),
                HierarchicalDeterministicFactorInstance::sample_other(),
            ]),
            1,
        )
    }

    fn sample_other() -> Self {
        Self::new(
            IndexSet::from_iter([
                HierarchicalDeterministicFactorInstance::sample_other(),
            ]),
            1,
        )
    }
}

impl PetitionForFactorsInput {
    pub(super) fn new(
        factors: IndexSet<HierarchicalDeterministicFactorInstance>,
        required: i8,
    ) -> Self {
        Self { factors, required }
    }

    pub(super) fn new_threshold(
        factors: IndexSet<HierarchicalDeterministicFactorInstance>,
        threshold: i8,
    ) -> Self {
        Self::new(factors, threshold)
    }

    pub(super) fn new_override(
        factors: IndexSet<HierarchicalDeterministicFactorInstance>,
    ) -> Self {
        Self::new(factors, 1) // we need just one, anyone, factor for threshold.
    }

    pub(crate) fn reference_factor_source_with_id(
        &self,
        factor_source_id: &FactorSourceIDFromHash,
    ) -> Option<&HierarchicalDeterministicFactorInstance> {
        self.factors
            .iter()
            .find(|f| f.factor_source_id == *factor_source_id)
    }

    fn factors_count(&self) -> i8 {
        self.factors.len() as i8
    }

    fn remaining_factors_until_success<ID: SignableID>(
        &self,
        snapshot: PetitionForFactorsStateSnapshot<ID>,
    ) -> i8 {
        self.required - snapshot.signed_count()
    }

    pub(super) fn is_fulfilled_by<ID: SignableID>(
        &self,
        snapshot: PetitionForFactorsStateSnapshot<ID>,
    ) -> bool {
        self.remaining_factors_until_success(snapshot) <= 0
    }

    fn factors_left_to_prompt<ID: SignableID>(
        &self,
        snapshot: PetitionForFactorsStateSnapshot<ID>,
    ) -> i8 {
        self.factors_count() - snapshot.prompted_count()
    }

    pub(super) fn is_failure_with<ID: SignableID>(
        &self,
        snapshot: PetitionForFactorsStateSnapshot<ID>,
    ) -> bool {
        let signed_or_pending = self.factors_left_to_prompt(snapshot.clone())
            + snapshot.signed_count();
        let is_failure_with = signed_or_pending < self.required;
        trace!(
            "is_failure_with: {:?}, signed_or_pending: {:?}, required: {:?}",
            is_failure_with,
            signed_or_pending,
            self.required
        );
        is_failure_with
    }
}
