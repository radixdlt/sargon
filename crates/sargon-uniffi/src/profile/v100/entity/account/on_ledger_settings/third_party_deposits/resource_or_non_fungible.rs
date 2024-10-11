use crate::prelude::*;
use sargon::ResourceOrNonFungible as InternalResourceOrNonFungible;

/// The addresses that can be added as exception to the `DepositRule`
#[derive(
    Clone,  PartialEq, Eq, Hash, uniffi::Enum,
)]
pub enum ResourceOrNonFungible {
    Resource { value: ResourceAddress },

    NonFungible { value: NonFungibleGlobalId },
}

impl From<InternalResourceOrNonFungible> for ResourceOrNonFungible {
    fn from(value: InternalResourceOrNonFungible) -> Self {
        match value {
            InternalResourceOrNonFungible::Resource { value } => ResourceOrNonFungible::Resource { value: value.into() },
            InternalResourceOrNonFungible::NonFungible { value } => ResourceOrNonFungible::NonFungible { value: value.into() },
        }
    }
}

impl Into<InternalResourceOrNonFungible> for ResourceOrNonFungible {
    fn into(self) -> InternalResourceOrNonFungible {
        match self {
            ResourceOrNonFungible::Resource { value } => InternalResourceOrNonFungible::Resource { value: value.into() },
            ResourceOrNonFungible::NonFungible { value } => InternalResourceOrNonFungible::NonFungible { value: value.into() },
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

