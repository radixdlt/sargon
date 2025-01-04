use crate::prelude::*;

/// A collection of derivation paths, on a per-factor-source basis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyDerivationRequest {
    /// We include this `DerivationPurpose` in dispatched use FactorSource requests to host so
    /// that UI can display contextual information as to why the user is prompted to
    /// authenticate FactorSource access.
    pub derivation_purpose: DerivationPurpose,
    pub per_factor_source:
        IndexMap<FactorSourceIDFromHash, IndexSet<DerivationPath>>,
}

impl KeyDerivationRequest {
    pub fn new(
        derivation_purpose: DerivationPurpose,
        per_factor_source: IndexMap<
            FactorSourceIDFromHash,
            IndexSet<DerivationPath>,
        >,
    ) -> Self {
        Self {
            derivation_purpose,
            per_factor_source,
        }
    }

    pub(crate) fn new_mono_factor(
        derivation_purpose: DerivationPurpose,
        factor_source: FactorSourceIDFromHash,
        derivation_paths: IndexSet<DerivationPath>,
    ) -> Self {
        Self::new(
            derivation_purpose,
            IndexMap::just((factor_source, derivation_paths)),
        )
    }
}
