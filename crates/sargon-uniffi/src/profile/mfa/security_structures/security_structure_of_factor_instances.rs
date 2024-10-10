use crate::prelude::*;
use sargon::SecurityStructureOfFactorInstances as InternalSecurityStructureOfFactorInstances;

decl_matrix_of_factors!(
    /// A matrix of FactorInstances
    FactorInstance
);

#[derive(
   Clone,  PartialEq, Eq, Hash,  uniffi::Record,
)]
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

impl From<InternalSecurityStructureOfFactorInstances> for SecurityStructureOfFactorInstances {
    fn from(value: InternalSecurityStructureOfFactorInstances) -> Self {
        Self {
            security_structure_id: value.security_structure_id.into(),
            matrix_of_factors: value.matrix_of_factors.into(),
        }
    }
}

impl Into<InternalSecurityStructureOfFactorInstances> for SecurityStructureOfFactorInstances {
    fn into(self) -> InternalSecurityStructureOfFactorInstances {
        InternalSecurityStructureOfFactorInstances {
            security_structure_id: self.security_structure_id.into(),
            matrix_of_factors: self.matrix_of_factors.into(),
        }
    }
}