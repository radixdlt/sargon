use crate::prelude::*;

/// A collection of derivation paths, on a per-factor-source basis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyDerivationRequest {
    pub per_factor_source: IndexMap<FactorSourceIDFromHash, IndexSet<DerivationPath>>
}

impl KeyDerivationRequest {
    pub(crate) fn new(
        per_factor_source: IndexMap<
            FactorSourceIDFromHash,
            IndexSet<DerivationPath>,
        >,
    ) -> Self {
        Self { per_factor_source }
    }

    pub(crate) fn new_mono_factor(
        factor_source: FactorSourceIDFromHash,
        derivation_paths: IndexSet<DerivationPath>,
    ) -> Self {
        Self::new(
            IndexMap::just((factor_source, derivation_paths))
        )
    }
}
