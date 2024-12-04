use crate::prelude::*;
use sargon::IndexMap;
use sargon::IndexSet;
use sargon::KeyDerivationResponse as InternalKeyDerivationResponse;

/// A collection of `HierarchicalDeterministicFactorInstance`s, on a
/// per-factor-source basis. In case of MonoKeyDerivation the list will contain
/// a single `KeyDerivationPerFactorSource`.
#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct KeyDerivationResponse {
    pub per_factor_source: Vec<KeyDerivationPerFactorSource>,
}

#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct KeyDerivationPerFactorSource {
    pub factor_source_id: FactorSourceIDFromHash,
    pub factor_instances: Vec<HierarchicalDeterministicFactorInstance>,
}

impl KeyDerivationPerFactorSource {
    pub fn new(
        factor_source_id: FactorSourceIDFromHash,
        factor_instances: Vec<HierarchicalDeterministicFactorInstance>,
    ) -> Self {
        Self { factor_source_id, factor_instances }
    }
}


impl KeyDerivationResponse {
    pub fn into_internal(&self) -> InternalKeyDerivationResponse {
        self.clone().into()
    }
}

impl From<InternalKeyDerivationResponse> for KeyDerivationResponse {
    fn from(value: InternalKeyDerivationResponse) -> Self {
        Self {
            per_factor_source: value
                .per_factor_source
                .into_iter()
                .map(|(k, v)| {
                    KeyDerivationPerFactorSource::new(
                        k.into(),
                        v.into_iter().map(|d| d.into()).collect()
                    )
                })
                .collect(),
        }
    }
}

impl From<KeyDerivationResponse> for InternalKeyDerivationResponse {
    fn from(value: KeyDerivationResponse) -> Self {
        Self::new(IndexMap::from_iter(
            value.per_factor_source.into_iter().map(|item| {
                (
                    item.factor_source_id.into_internal(),
                    IndexSet::from_iter(
                        item.factor_instances.into_iter().map(|d| d.into_internal()),
                    ),
                )
            }),
        ))
    }
}
