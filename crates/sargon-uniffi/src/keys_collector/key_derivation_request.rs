use crate::prelude::*;
use sargon::indexmap::IndexMap;
use sargon::{IndexSet, KeyDerivationRequest as InternalKeyDerivationRequest};

/// A collection of derivation paths, on a per-factor-source basis.
#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct KeyDerivationRequest {
    pub per_factor_source: HashMap<FactorSourceIDFromHash, Vec<DerivationPath>>,
}

impl KeyDerivationRequest {
    pub fn into_internal(&self) -> InternalKeyDerivationRequest {
        self.clone().into()
    }
}

impl From<InternalKeyDerivationRequest> for KeyDerivationRequest {
    fn from(value: InternalKeyDerivationRequest) -> Self {
        Self {
            per_factor_source: value
                .per_factor_source
                .into_iter()
                .map(|(k, v)| {
                    (k.into(), v.into_iter().map(|d| d.into()).collect())
                })
                .collect(),
        }
    }
}

impl From<KeyDerivationRequest> for InternalKeyDerivationRequest {
    fn from(value: KeyDerivationRequest) -> Self {
        Self::new(IndexMap::from_iter(
            value.per_factor_source.into_iter().map(|(k, v)| {
                (
                    k.into_internal(),
                    IndexSet::from_iter(
                        v.into_iter().map(|d| d.into_internal()),
                    ),
                )
            }),
        ))
    }
}
