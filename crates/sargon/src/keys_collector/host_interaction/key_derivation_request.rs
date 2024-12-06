use crate::prelude::*;

/// A collection of derivation paths, on a per-factor-source basis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyDerivationRequest {
    pub keys_collection_reason: KeysCollectionReason,
    pub per_factor_source:
        IndexMap<FactorSourceIDFromHash, IndexSet<DerivationPath>>,
}

impl KeyDerivationRequest {
    pub fn new(
        keys_collection_reason: KeysCollectionReason,
        per_factor_source: IndexMap<
            FactorSourceIDFromHash,
            IndexSet<DerivationPath>,
        >,
    ) -> Self {
        Self {
            keys_collection_reason,
            per_factor_source,
        }
    }

    pub(crate) fn new_mono_factor(
        keys_collection_reason: KeysCollectionReason,
        factor_source: FactorSourceIDFromHash,
        derivation_paths: IndexSet<DerivationPath>,
    ) -> Self {
        Self::new(
            keys_collection_reason,
            IndexMap::just((factor_source, derivation_paths)),
        )
    }
}
