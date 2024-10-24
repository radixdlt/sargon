use crate::prelude::*;

decl_security_structure_of!(
    /// A security structure at FactorSourceID level, this is
    /// what is serialized and store into Profile, we convert
    /// into this structure from `SecurityStructureOfFactorSources`.
    FactorSourceID,
);

decl_vec_samples_for!(
    SecurityStructuresOfFactorSourceIDs,
    SecurityStructureOfFactorSourceIDs
);

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
