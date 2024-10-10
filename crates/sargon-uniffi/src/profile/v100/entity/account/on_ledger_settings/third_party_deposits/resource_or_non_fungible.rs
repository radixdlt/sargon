use crate::prelude::*;
use sargon::ResourceOrNonFungible as InternalResourceOrNonFungible;

/// The addresses that can be added as exception to the `DepositRule`
#[derive(
    Clone, Debug, PartialEq, Eq, Hash, uniffi::Enum,
)]
pub enum ResourceOrNonFungible {
    Resource { value: ResourceAddress },

    NonFungible { value: NonFungibleGlobalId },
}

impl From<ResourceOrNonFungible> for InternalResourceOrNonFungible {
    fn from(value: ResourceOrNonFungible) -> Self {
        match value {
            ResourceOrNonFungible::Resource { value } => InternalResourceOrNonFungible::Resource(value.into()),
            ResourceOrNonFungible::NonFungible { value } => InternalResourceOrNonFungible::NonFungible(value.into()),
        }
    }
}

impl Into<ResourceOrNonFungible> for InternalResourceOrNonFungible {
    fn into(self) -> ResourceOrNonFungible {
        match self {
            InternalResourceOrNonFungible::Resource(value) => ResourceOrNonFungible::Resource { value: value.into() },
            InternalResourceOrNonFungible::NonFungible(value) => ResourceOrNonFungible::NonFungible { value: value.into() },
        }
    }
}

#[uniffi::export]
pub fn new_resource_or_non_fungible_sample() -> ResourceOrNonFungible {
    InternalResourceOrNonFungible::sample().into()
}

#[uniffi::export]
pub fn new_resource_or_non_fungible_sample_other() -> ResourceOrNonFungible {
    InternalResourceOrNonFungible::sample_other().into()
}

