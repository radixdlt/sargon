use crate::prelude::*;
use sargon::IndexMap;
use sargon::IndexSet;
use sargon::KeyDerivationResponse as InternalKeyDerivationResponse;

#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct KeyDerivationResponse {
    pub per_factor_source: HashMap<
        FactorSourceIDFromHash,
        Vec<HierarchicalDeterministicFactorInstance>,
    >,
}


impl KeyDerivationResponse {

    pub fn into_internal(&self) -> InternalKeyDerivationResponse {
        self.clone().into()
    }

}

impl From<InternalKeyDerivationResponse> for KeyDerivationResponse {
    fn from(value: InternalKeyDerivationResponse) -> Self {
        Self {
            per_factor_source: value.per_factor_source
                .into_iter()
                .map(|(k, v)| (k.into(), v.into_iter().map(|d| d.into()).collect()))
                .collect()
        }
    }
}

impl From<KeyDerivationResponse> for InternalKeyDerivationResponse {
    fn from(value: KeyDerivationResponse) -> Self {
        Self::new(
            IndexMap::from_iter(
                value.per_factor_source
                    .into_iter()
                    .map(|(k, v)| (k.into_internal(), IndexSet::from_iter(v.into_iter().map(|d| d.into_internal()))))
            )
        )
    }
}