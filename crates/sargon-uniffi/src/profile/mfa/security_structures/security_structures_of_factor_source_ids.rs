use crate::prelude::*;

decl_identified_vec_of!(
    /// A collection of [`SecurityStructureOfFactorSourceIDs`]
    SecurityStructuresOfFactorSourceIDs,
    SecurityStructureOfFactorSourceIDs
);

#[uniffi::export]
pub fn new_security_structure_of_factor_source_ids_sample(
) -> SecurityStructureOfFactorSourceIDs {
    SecurityStructureOfFactorSourceIDs::sample()
}

#[uniffi::export]
pub fn new_security_structure_of_factor_source_ids_sample_other(
) -> SecurityStructureOfFactorSourceIDs {
    SecurityStructureOfFactorSourceIDs::sample_other()
}

