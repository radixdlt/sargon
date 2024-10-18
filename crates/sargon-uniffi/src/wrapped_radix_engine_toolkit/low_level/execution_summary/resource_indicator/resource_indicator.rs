use crate::prelude::*;
use sargon::ResourceIndicator as InternalResourceIndicator;

#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
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
