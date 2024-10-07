use crate::prelude::*;

/// A sub-state of `PetitionForFactorsState` which can be used to track factors
/// that have signed or skipped.
#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
#[debug("[{}]", factors.borrow().clone().into_iter().map(|f| format!("{:?}", f)).join(", "))]
pub(crate) struct PetitionForFactorsSubState<F>
where
    F: FactorSourceReferencing + Debug,
{
    /// Factors that have signed or skipped
    factors: RefCell<IndexSet<F>>,
}

impl<F: FactorSourceReferencing + Debug> PetitionForFactorsSubState<F> {
    pub(super) fn new() -> Self {
        Self {
            factors: RefCell::new(IndexSet::new()),
        }
    }

    pub(super) fn insert(&self, factor: &F) {
        self.factors.borrow_mut().insert(factor.clone());
    }

    pub(super) fn snapshot(&self) -> IndexSet<F> {
        self.factors.borrow().clone()
    }

    pub(super) fn references_factor_source_by_id(
        &self,
        factor_source_id: FactorSourceIDFromHash,
    ) -> bool {
        self.factors
            .borrow()
            .iter()
            .any(|sf| sf.factor_source_id() == factor_source_id)
    }
}
