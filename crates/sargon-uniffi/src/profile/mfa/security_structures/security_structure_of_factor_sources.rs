use crate::prelude::*;

decl_security_structure_of!(
    /// Security structure at `FactorSource` level.
    /// This is what user view, creates and manages.
    ///
    /// Before it gets saved into Profile gets converted into
    /// `SecurityStructureOfFactorSourceIDs`
    FactorSource,
);

#[uniffi::export]
pub fn new_security_structure_of_factor_sources_sample(
) -> SecurityStructureOfFactorSources {
    SecurityStructureOfFactorSources::sample()
}

#[uniffi::export]
pub fn new_security_structure_of_factor_sources_sample_other(
) -> SecurityStructureOfFactorSources {
    SecurityStructureOfFactorSources::sample_other()
}

#[uniffi::export]
pub fn new_security_structure_of_factor_sources_auto_in_days(
    metadata: SecurityStructureMetadata,
    number_of_days_until_auto_confirmation: u16,
    matrix_of_factors: MatrixOfFactorSources,
) -> SecurityStructureOfFactorSources {
    SecurityStructureOfFactorSources::new_with_days(
        metadata,
        number_of_days_until_auto_confirmation,
        matrix_of_factors,
    )
}

