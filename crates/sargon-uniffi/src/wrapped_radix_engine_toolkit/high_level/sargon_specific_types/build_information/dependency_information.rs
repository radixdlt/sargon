use crate::prelude::*;
use sargon::DependencyInformation as InternalDependencyInformation;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum DependencyInformation {
    // Crates.io
    Version(String),

    // Github
    Tag(String),
    Branch(String),
    Rev(String),
}

impl From<InternalDependencyInformation> for DependencyInformation {
    fn from(value: InternalDependencyInformation) -> Self {
        match value {
            InternalDependencyInformation::Version(value) => {
                DependencyInformation::Version(value)
            }
            InternalDependencyInformation::Tag(value) => {
                DependencyInformation::Tag(value)
            }
            InternalDependencyInformation::Branch(value) => {
                DependencyInformation::Branch(value)
            }
            InternalDependencyInformation::Rev(value) => {
                DependencyInformation::Rev(value)
            }
        }
    }
}

impl Into<InternalDependencyInformation> for DependencyInformation {
    fn into(self) -> InternalDependencyInformation {
        match self {
            DependencyInformation::Version(value) => {
                InternalDependencyInformation::Version(value)
            }
            DependencyInformation::Tag(value) => {
                InternalDependencyInformation::Tag(value)
            }
            DependencyInformation::Branch(value) => {
                InternalDependencyInformation::Branch(value)
            }
            DependencyInformation::Rev(value) => {
                InternalDependencyInformation::Rev(value)
            }
        }
    }
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
