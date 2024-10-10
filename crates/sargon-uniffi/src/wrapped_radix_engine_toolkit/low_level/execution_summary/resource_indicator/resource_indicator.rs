use crate::prelude::*;
use sargon::ResourceIndicator as InternalResourceIndicator;

#[derive(Clone,  PartialEq, Eq, EnumAsInner, uniffi::Enum)]
pub enum ResourceIndicator {
    Fungible {
        resource_address: ResourceAddress,
        indicator: FungibleResourceIndicator,
    },
    NonFungible {
        resource_address: ResourceAddress,
        indicator: NonFungibleResourceIndicator,
    },
}

impl From<InternalResourceIndicator> for ResourceIndicator {
    fn from(value: InternalResourceIndicator) -> Self {
        match value {
            InternalResourceIndicator::Fungible {
                resource_address,
                indicator,
            } => ResourceIndicator::Fungible {
                resource_address: resource_address.into(),
                indicator: indicator.into(),
            },
            InternalResourceIndicator::NonFungible {
                resource_address,
                indicator,
            } => ResourceIndicator::NonFungible {
                resource_address: resource_address.into(),
                indicator: indicator.into(),
            },
        }
    }
}

impl Into<InternalResourceIndicator> for ResourceIndicator {
    fn into(self) -> InternalResourceIndicator {
        match self {
            ResourceIndicator::Fungible {
                resource_address,
                indicator,
            } => InternalResourceIndicator::Fungible {
                resource_address: resource_address.into(),
                indicator: indicator.into(),
            },
            ResourceIndicator::NonFungible {
                resource_address,
                indicator,
            } => InternalResourceIndicator::NonFungible {
                resource_address: resource_address.into(),
                indicator: indicator.into(),
            },
        }
    }
}

#[uniffi::export]
pub fn new_resource_indicator_sample() -> ResourceIndicator {
    InternalResourceIndicator::sample().into()
}

#[uniffi::export]
pub fn new_resource_indicator_sample_other() -> ResourceIndicator {
    InternalResourceIndicator::sample_other().into()
}

#[uniffi::export]
pub fn resource_indicator_get_address(
    indicator: &ResourceIndicator,
) -> ResourceAddress {
    indicator.into_internal().get_address().into()
}

