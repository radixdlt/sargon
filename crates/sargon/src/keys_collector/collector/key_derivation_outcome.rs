use crate::prelude::*;

/// A collection of all `HierarchicalDeterministicFactorInstance`
/// (Public Keys) which were derived from the referenced
/// `FactorSource`s at the specified `DerivationPath`s
#[derive(PartialEq, Eq, Clone, derive_more::Debug)]
pub struct KeyDerivationOutcome {
    #[doc(hidden)]
    #[debug(skip)]
    #[allow(dead_code)]
    __hidden: HiddenConstructor,
    pub factors_by_source: IndexMap<
        FactorSourceIDFromHash,
        IndexSet<HierarchicalDeterministicFactorInstance>,
    >,
}

impl KeyDerivationOutcome {
    pub(crate) fn new(
        factors_by_source: IndexMap<
            FactorSourceIDFromHash,
            IndexSet<HierarchicalDeterministicFactorInstance>,
        >,
    ) -> Self {
        Self {
            __hidden: HiddenConstructor,
            factors_by_source,
        }
    }

    /// ALL factor instances derived by the KeysCollector
    pub fn all_factors(&self) -> FactorInstances {
        self.factors_by_source
            .clone()
            .into_iter()
            .flat_map(|(_, v)| v)
            .collect()
    }
}
