use crate::prelude::*;
use sargon::DependencyInformation as InternalDependencyInformation;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
pub enum DependencyInformation {
    // Crates.io
    Version(String),

    // Github
    Tag(String),
    Branch(String),
    Rev(String),
}

#[uniffi::export]
pub fn new_dependency_information_sample() -> DependencyInformation {
    InternalDependencyInformation::sample().into()
}

#[uniffi::export]
pub fn new_dependency_information_sample_other() -> DependencyInformation {
    InternalDependencyInformation::sample_other().into()
}

#[uniffi::export]
pub fn dependency_information_to_string(
    info: &DependencyInformation,
) -> String {
    info.into_internal().to_string()
}
