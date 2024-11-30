use crate::prelude::*;

#[uniffi::export]
pub fn new_security_structure_of_factor_sources_sample(
) -> SecurityStructureOfFactorSources {
    InternalSecurityStructureOfFactorSources::sample().into()
}

#[uniffi::export]
pub fn new_security_structure_of_factor_sources_sample_other(
) -> SecurityStructureOfFactorSources {
    InternalSecurityStructureOfFactorSources::sample_other().into()
}

#[uniffi::export]
pub fn new_security_structure_of_factor_sources_auto_in_days(
    metadata: SecurityStructureMetadata,
    number_of_days_until_auto_confirmation: u16,
    matrix_of_factors: MatrixOfFactorSources,
) -> SecurityStructureOfFactorSources {
    InternalSecurityStructureOfFactorSources::new_with_days(
        metadata.into_internal(),
        number_of_days_until_auto_confirmation,
        matrix_of_factors.into_internal(),
    )
    .into()
}

#[uniffi::export]
pub fn new_matrix_of_factor_sources_sample() -> MatrixOfFactorSources {
    InternalMatrixOfFactorSources::sample().into()
}

#[uniffi::export]
pub fn new_matrix_of_factor_sources_sample_other() -> MatrixOfFactorSources {
    InternalMatrixOfFactorSources::sample_other().into()
}
