use crate::prelude::*;
use sargon::SecurityStructureOfFactorSourceIDs as InternalSecurityStructureOfFactorSourceIDs;

#[uniffi::export]
pub fn new_security_structure_of_factor_source_ids_sample(
) -> SecurityStructureOfFactorSourceIDs {
    InternalSecurityStructureOfFactorSourceIDs::sample().into()
}

#[uniffi::export]
pub fn new_security_structure_of_factor_source_ids_sample_other(
) -> SecurityStructureOfFactorSourceIDs {
    InternalSecurityStructureOfFactorSourceIDs::sample_other().into()
}
