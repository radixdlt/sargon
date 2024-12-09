use crate::prelude::*;
use sargon::IndexMap;
use sargon::{IndexSet, KeyDerivationRequest as InternalKeyDerivationRequest};

/// A collection of derivation paths, on a per-factor-source basis.
#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct KeyDerivationRequest {
    /// We include this `DerivationPurpose` in dispatched use FactorSource requests to host so
    /// that UI can display contextual information as to why the user is prompted to
    /// authenticate FactorSource access.
    pub derivation_purpose: DerivationPurpose,
    pub per_factor_source: Vec<KeyDerivationRequestPerFactorSource>,
}

#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct KeyDerivationRequestPerFactorSource {
    pub factor_source_id: FactorSourceIDFromHash,
    pub derivation_paths: Vec<DerivationPath>,
}

impl KeyDerivationRequestPerFactorSource {
    pub fn new(
        factor_source_id: FactorSourceIDFromHash,
        derivation_paths: Vec<DerivationPath>,
    ) -> Self {
        Self {
            factor_source_id,
            derivation_paths,
        }
    }
}

impl KeyDerivationRequest {
    pub fn into_internal(&self) -> InternalKeyDerivationRequest {
        self.clone().into()
    }
}

impl From<InternalKeyDerivationRequest> for KeyDerivationRequest {
    fn from(value: InternalKeyDerivationRequest) -> Self {
        Self {
            derivation_purpose: value.derivation_purpose.into(),
            per_factor_source: value
                .per_factor_source
                .into_iter()
                .map(|(k, v)| {
                    KeyDerivationRequestPerFactorSource::new(
                        k.into(),
                        v.into_iter().map(|d| d.into()).collect(),
                    )
                })
                .collect(),
        }
    }
}

impl From<KeyDerivationRequest> for InternalKeyDerivationRequest {
    fn from(value: KeyDerivationRequest) -> Self {
        Self::new(
            value.derivation_purpose.into_internal(),
            IndexMap::from_iter(value.per_factor_source.into_iter().map(|f| {
                (
                    f.factor_source_id.into_internal(),
                    IndexSet::from_iter(
                        f.derivation_paths
                            .into_iter()
                            .map(|d| d.into_internal()),
                    ),
                )
            })),
        )
    }
}
