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

    #[error("The number of days until timed confirm is callable must be greater than zero")]
    NumberOfDaysUntilTimeBasedConfirmationMustBeGreaterThanZero,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
pub enum MatrixRolesInCombinationForeverInvalid {
    #[error("Recovery and confirmation factors overlap. No factor may be used in both the recovery and confirmation roles")]
    RecoveryAndConfirmationFactorsOverlap,
    #[error("Primary role cannot have multiple devices")]
    PrimaryCannotHaveMultipleDevices,
    #[error("Threshold and override factors overlap. No factor may be used in both the threshold and override list kinds")]
    ThresholdAndOverrideFactorsOverlap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
pub enum MatrixRolesInCombinationNotYetValid {
    #[error("The single factor used in the primary role must not be used in any other role")]
    SingleFactorUsedInPrimaryMustNotBeUsedInAnyOtherRole,
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
