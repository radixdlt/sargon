use crate::prelude::*;
use sargon::SargonBuildInformation as InternalSargonBuildInformation;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Record)]
pub struct SargonBuildInformation {
    pub sargon_version: String,
    pub dependencies: SargonDependencies,
}

#[uniffi::export]
pub fn new_sargon_build_information_sample() -> SargonBuildInformation {
    InternalSargonBuildInformation::sample().into()
}

#[uniffi::export]
pub fn new_sargon_build_information_sample_other() -> SargonBuildInformation {
    InternalSargonBuildInformation::sample_other().into()
}
