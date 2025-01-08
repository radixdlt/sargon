use crate::prelude::*;

/// A sub-state of `PetitionForFactorsState` which can be used to track factors
/// that have signed or skipped.
#[derive(derive_more::Debug)]
#[debug("[{}]", self.snapshot().into_iter().map(|f| format!("{:?}", f)).join(", "))]
pub(crate) struct PetitionForFactorsSubState<F>
where
    F: FactorSourceReferencing + Debug,
{
    /// Factors that have signed or skipped
    factors: RwLock<IndexSet<F>>,
}

impl<F: FactorSourceReferencing + Debug> PartialEq
    for PetitionForFactorsSubState<F>
{
    fn eq(&self, other: &Self) -> bool {
        let self_state = self.snapshot();
        let other_state = other.snapshot();

        self_state == other_state
    }
}

impl<F: FactorSourceReferencing + Debug> Eq for PetitionForFactorsSubState<F> {}

impl<F: FactorSourceReferencing + Debug + HasSampleValues> HasSampleValues
    for PetitionForFactorsSubState<F>
{
    fn sample() -> Self {
        let state = PetitionForFactorsSubState::new();
        let sample = F::sample();
        state.insert(&sample);
        state
    }

    fn sample_other() -> Self {
        let state = PetitionForFactorsSubState::new();
        let sample = F::sample_other();
        state.insert(&sample);
        state
    }
}

impl<F: FactorSourceReferencing + Debug> PetitionForFactorsSubState<F> {
    pub(super) fn new() -> Self {
        Self {
            factors: RwLock::new(IndexSet::new()),
        }
    }

    pub(super) fn insert(&self, factor: &F) {
        self.factors
            .write()
            .expect("PetitionForFactorsSubState should not have been poisoned")
            .insert(factor.clone());
    }

    pub(super) fn snapshot(&self) -> IndexSet<F> {
        self.factors
            .read()
            .expect("PetitionForFactorsSubState should not have been poisoned")
            .clone()
    }

    pub(super) fn references_factor_source_by_id(
        &self,
        factor_source_id: FactorSourceIDFromHash,
    ) -> bool {
        self.factors
            .read()
            .expect("PetitionForFactorsSubState should not have been poisoned")
            .iter()
            .any(|sf| sf.factor_source_id() == factor_source_id)
    }

    pub(super) fn cloned(&self) -> Self {
        Self {
            factors: RwLock::new(self.snapshot()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PetitionForFactorsSubState<NeglectedFactorInstance>;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
