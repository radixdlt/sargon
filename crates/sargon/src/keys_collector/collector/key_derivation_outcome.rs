use crate::prelude::*;

/// A collection of all `HierarchicalDeterministicFactorInstance`
/// (Public Keys) which were derived from the referenced
/// `FactorSource`s at the specified `DerivationPath`s
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct KeyDerivationOutcome {
    count: usize, // hide ctor...
    pub factors_by_source:
        IndexMap<FactorSourceIDFromHash, IndexSet<HierarchicalDeterministicFactorInstance>>,
}

impl KeyDerivationOutcome {
    pub(crate) fn new(
        factors_by_source: IndexMap<
            FactorSourceIDFromHash,
            IndexSet<HierarchicalDeterministicFactorInstance>,
        >,
    ) -> Self {
        Self {
            count: factors_by_source.len(),
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
