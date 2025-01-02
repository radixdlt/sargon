use crate::prelude::*;

use sargon::SecurityStructureOfFactorSources as InternalSecurityStructureOfFactorSources;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct SecurityStructureOfFactorSources {
    /// Metadata of this Security Structure, such as globally unique and
    /// stable identifier, creation date and user chosen label (name).
    pub metadata: SecurityStructureMetadata,

    /// The structure of factors to use for certain roles, Primary, Recovery
    /// and Confirmation role.
    pub matrix_of_factors: MatrixOfFactorSources,

    /// The factor to use for authentication signing aka true Rola Key.
    pub authentication_signing_factor: FactorSource,
}

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
