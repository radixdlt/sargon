use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
pub enum MatrixRolesInCombinationViolation {
    #[error("Basic violation: {0}")]
    Basic(#[from] MatrixRolesInCombinationBasicViolation),

    #[error("Forever invalid: {0}")]
    ForeverInvalid(#[from] MatrixRolesInCombinationForeverInvalid),

    #[error("Not yet valid: {0}")]
    NotYetValid(#[from] MatrixRolesInCombinationNotYetValid),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
pub enum MatrixRolesInCombinationBasicViolation {
    #[error("The factor source was not found in any role")]
    FactorSourceNotFoundInAnyRole,

    #[error("The number of days until auto confirm must be greater than zero")]
    NumberOfDaysUntilAutoConfirmMustBeGreaterThanZero,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
pub enum MatrixRolesInCombinationForeverInvalid {
    #[error("Recovery and confirmation factors overlap. No factor may be used in both the recovery and confirmation roles")]
    RecoveryAndConfirmationFactorsOverlap,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    enum_iterator::Sequence,
    thiserror::Error,
)]
pub enum MatrixRolesInCombinationNotYetValid {
    #[error("The single factor used in the primary role must not be used in any other role")]
    SingleFactorUsedInPrimaryMustNotBeUsedInAnyOtherRole,
}

impl MatrixRolesInCombinationNotYetValid {
    pub fn all() -> Vec<Self> {
        enum_iterator::all::<Self>().collect()
    }
}

impl CommonError {
    /// Checks if this CommonError (self) is a MatrixBuilderValidation::MatrixRolesInCombinationNotYetValid violation, which
    /// is in fact not a real error.
    pub fn is_matrix_builder_not_yet_valid(&self) -> bool {
        for not_yet_valid_reason in MatrixRolesInCombinationNotYetValid::all() {
            let as_common_error = Self::from(not_yet_valid_reason);
            if *self == as_common_error {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
pub enum MatrixBuilderValidation {
    #[error("Role {role:?} in isolation violation: {violation}")]
    RoleInIsolation {
        role: RoleKind,
        violation: RoleBuilderValidation,
    },
    #[error("Roles in combination violation: {0}")]
    CombinationViolation(#[from] MatrixRolesInCombinationViolation),
}

pub(crate) trait IntoMatrixErr<T> {
    fn into_matrix_err(
        self,
        role: RoleKind,
    ) -> Result<T, MatrixBuilderValidation>;
}

impl<T> IntoMatrixErr<T> for Result<T, RoleBuilderValidation> {
    fn into_matrix_err(
        self,
        role: RoleKind,
    ) -> Result<T, MatrixBuilderValidation> {
        self.map_err(|violation| MatrixBuilderValidation::RoleInIsolation {
            role,
            violation,
        })
    }
}
