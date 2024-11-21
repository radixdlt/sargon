use crate::prelude::*;

/// A collection of `IndexSet<HierarchicalDeterministicFactorInstance>`, on a
/// per-factor-source basis. In case of MonoKeyDerivation the map will contain
/// a single tuple.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyDerivationResponse {
    pub per_factor_source: IndexMap<
        FactorSourceIDFromHash,
        IndexSet<HierarchicalDeterministicFactorInstance>,
    >,
}

impl KeyDerivationResponse {
    pub fn new(
        per_factor_source: IndexMap<
            FactorSourceIDFromHash,
            IndexSet<HierarchicalDeterministicFactorInstance>,
        >,
    ) -> Self {
        Self { per_factor_source }
    }
}
