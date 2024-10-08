use crate::prelude::*;
use sargon::DependencyInformation as InternalDependencyInformation;

#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    uniffi::Enum,
)]
pub enum DependencyInformation {
    // Crates.io
    Version(String),

    // Github
    Tag(String),
    Branch(String),
    Rev(String),
}

impl From<InternalDependencyInformation> for DependencyInformation {
    fn from(value: DependencyInformation) -> Self {
        match value {
            InternalDependencyInformation::Version(value) => DependencyInformation::Version(value),
            InternalDependencyInformation::Tag(value) => DependencyInformation::Tag(value),
            InternalDependencyInformation::Branch(value) => DependencyInformation::Branch(value),
            InternalDependencyInformation::Rev(value) => DependencyInformation::Rev(value),
        }
    }
}

impl Into<InternalDependencyInformation> for DependencyInformation {
    fn into(self) -> InternalDependencyInformation {
        match self {
            DependencyInformation::Version(value) => InternalDependencyInformation::Version(value),
            DependencyInformation::Tag(value) => InternalDependencyInformation::Tag(value),
            DependencyInformation::Branch(value) => InternalDependencyInformation::Branch(value),
            DependencyInformation::Rev(value) => InternalDependencyInformation::Rev(value),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DependencyInformation;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_dependency_information_sample(),
                new_dependency_information_sample_other(),
                // duplicates should get removed
                new_dependency_information_sample(),
                new_dependency_information_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn to_string() {
        assert_eq!(dependency_information_to_string(&SUT::sample()), "main");
    }
}
