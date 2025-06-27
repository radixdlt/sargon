use crate::prelude::*;
use sargon::SecurityStructureOfFactorInstances as InternalSecurityStructureOfFactorInstances;

/// A MatrixOfFactorInstances and an ID which identifies it, this is
/// the Profile data structure representation of the owner key hashes which
/// have been uploaded as Scrypto AccessRules on the AccessController on-ledger.
///
/// Also contains an authentication signing factor instance which is used for
/// Rola.
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

    /// The authentication signing factor instance which is used to sign
    /// proof of ownership - aka "True Rola Key". User can select which FactorSource
    /// to use during Shield Building, but typically most users will use the
    /// DeviceFactorSource which is default. DerivationPath is in securified
    /// KeySpace of course.
    ///
    /// Non-optional since we can replace it with a new one for entities
    /// we have recovered during Onboarding Account Recovery Scan for securified
    /// entities
    pub authentication_signing_factor_instance:
        HierarchicalDeterministicFactorInstance,
}

#[uniffi::export]
pub fn new_security_structure_of_factor_instances_sample(
) -> SecurityStructureOfFactorInstances {
    InternalSecurityStructureOfFactorInstances::sample().into()
}

#[uniffi::export]
pub fn new_security_structure_of_factor_instances_sample_other(
) -> SecurityStructureOfFactorInstances {
    InternalSecurityStructureOfFactorInstances::sample_other().into()
}
