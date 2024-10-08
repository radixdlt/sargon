use crate::prelude::*;
use sargon::ResourceIdentifier as InternalResourceIdentifier;

/// An enum representation of an resource for which the user can set up its preferences.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    uniffi::Enum,
)]
pub enum ResourceIdentifier {
    Fungible(ResourceAddress),
    NonFungible(ResourceAddress),
    PoolUnit(PoolAddress),
}

impl From<InternalResourceIdentifier> for ResourceIdentifier {
    fn from(value: InternalResourceIdentifier) -> Self {
        match value {
            InternalResourceIdentifier::Fungible { value } => ResourceIdentifier::Fungible {
                value: value.into(),
            },
            InternalResourceIdentifier::NonFungible { value } => ResourceIdentifier::NonFungible {
                value: value.into(),
            },
            InternalResourceIdentifier::PoolUnit { value } => ResourceIdentifier::PoolUnit {
                value: value.into(),
            },
        }
    }
}

impl Into<InternalResourceIdentifier> for ResourceIdentifier {
    fn into(self) -> InternalResourceIdentifier {
        match self {
            ResourceIdentifier::Fungible { value } => InternalResourceIdentifier::Fungible {
                value: value.into(),
            },
            ResourceIdentifier::NonFungible { value } => InternalResourceIdentifier::NonFungible {
                value: value.into(),
            },
            ResourceIdentifier::PoolUnit { value } => InternalResourceIdentifier::PoolUnit {
                value: value.into(),
            },
        }
    }
}