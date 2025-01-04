use crate::prelude::*;

use sargon::SecurityStructureOfFactorSourceIDs as InternalSecurityStructureOfFactorSourceIDs;

pub type MatrixOfFactorSourceIds = MatrixOfFactorSourceIDs;

/// A `MatrixOfFactorSourceIDs` and associated metadata, this is
/// the Profile data structure representation of a "SecurityShield".
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct SecurityStructureOfFactorSourceIDs {
    /// Metadata of this Security Structure, such as globally unique and
    /// stable identifier, creation date and user chosen label (name).
    pub metadata: SecurityStructureMetadata,

    /// The structure of factors to use for certain roles, Primary, Recovery
    /// and Confirmation role.
    pub matrix_of_factors: MatrixOfFactorSourceIDs,

    /// The factor to use for authentication signing aka true Rola Key.
    pub authentication_signing_factor: FactorSourceID,
}

delegate_debug_into!(
    SecurityStructureOfFactorSourceIDs,
    InternalSecurityStructureOfFactorSourceIDs
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
