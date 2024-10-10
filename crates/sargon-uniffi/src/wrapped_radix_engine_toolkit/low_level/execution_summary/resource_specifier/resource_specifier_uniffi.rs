use crate::prelude::*;

#[uniffi::export]
pub fn new_resource_specifier_sample() -> ResourceSpecifier {
    ResourceSpecifier::sample()
}

#[uniffi::export]
pub fn new_resource_specifier_sample_other() -> ResourceSpecifier {
    ResourceSpecifier::sample_other()
}

#[uniffi::export]
pub fn resource_specifier_get_address(
    specifier: &ResourceSpecifier,
) -> ResourceAddress {
    specifier.get_address()
}

