use crate::prelude::*;

/// A collection of `MonoFactorKeyDerivationRequest`, on a per-factor-source basis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolyFactorKeyDerivationRequest {
    pub per_factor_source: IndexMap<FactorSourceIDFromHash, MonoFactorKeyDerivationRequest>,
}

impl PolyFactorKeyDerivationRequest {
    pub(crate) fn new(
        per_factor_source: IndexMap<FactorSourceIDFromHash, MonoFactorKeyDerivationRequest>,
    ) -> Self {
        Self { per_factor_source }
    }
}
