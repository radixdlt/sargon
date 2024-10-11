use crate::prelude::*;
use sargon::ResourceSpecifier as InternalResourceSpecifier;

#[derive(Clone, PartialEq, Eq, InternalConversionV2, uniffi::Enum)]
pub enum ResourceSpecifier {
    Fungible {
        resource_address: ResourceAddress,
        amount: Decimal,
    },
    NonFungible {
        resource_address: ResourceAddress,
        ids: Vec<NonFungibleLocalId>,
    },
}

#[uniffi::export]
pub fn new_resource_specifier_sample() -> ResourceSpecifier {
    InternalResourceSpecifier::sample().into()
}

#[uniffi::export]
pub fn new_resource_specifier_sample_other() -> ResourceSpecifier {
    InternalResourceSpecifier::sample_other().into()
}

#[uniffi::export]
pub fn resource_specifier_get_address(
    specifier: &ResourceSpecifier,
) -> ResourceAddress {
    specifier.into_internal().get_address().into()
}
