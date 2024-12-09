use crate::prelude::*;
use sargon::SecurityStructureOfFactorInstances as InternalSecurityStructureOfFactorInstances;

/// A MatrixOfFactorInstances and an ID which identifies it, this is
/// the Profile data structure representation of the owner key hashes which
/// have been uploaded as Scrypto AccessRules on the AccessController on-ledger.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct SecurityStructureOfFactorInstances {
    /// The ID of the `SecurityStructureOfFactorSourceIDs` in
    /// `profile.app_preferences.security.security_structures_of_factor_source_ids`
    /// which was used to derive the factor instances in this structure. Or rather:
    /// The id of `SecurityStructureOfFactorSources`.
    pub security_structure_id: SecurityStructureID,

    /// The structure of factors to use for certain roles, Primary, Recovery
    /// and Confirmation role.
    pub matrix_of_factors: MatrixOfFactorInstances,
}
