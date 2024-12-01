use crate::prelude::*;

impl From<MatrixRolesInCombinationBasicViolation> for CommonError {
    fn from(value: MatrixRolesInCombinationBasicViolation) -> Self {
        use MatrixRolesInCombinationBasicViolation::*;
        match value {
            FactorSourceNotFoundInAnyRole => {
                CommonError::FactorSourceNotFoundInAnyRole
            }
            NumberOfDaysUntilAutoConfirmMustBeGreaterThanZero => {
                CommonError::NumberOfDaysUntilAutoConfirmMustBeGreaterThanZero
            }
        }
    }
}
impl From<MatrixRolesInCombinationForeverInvalid> for CommonError {
    fn from(value: MatrixRolesInCombinationForeverInvalid) -> Self {
        use MatrixRolesInCombinationForeverInvalid::*;
        match value {
            RecoveryAndConfirmationFactorsOverlap => {
                Self::RecoveryAndConfirmationFactorsOverlap
            }
        }
    }
}
impl From<MatrixRolesInCombinationNotYetValid> for CommonError {
    fn from(value: MatrixRolesInCombinationNotYetValid) -> Self {
        use MatrixRolesInCombinationNotYetValid::*;

        match value {
            SingleFactorUsedInPrimaryMustNotBeUsedInAnyOtherRole => {
                Self::SingleFactorUsedInPrimaryMustNotBeUsedInAnyOtherRole
            }
        }
    }
}

impl From<MatrixRolesInCombinationViolation> for CommonError {
    fn from(value: MatrixRolesInCombinationViolation) -> Self {
        match value {
            MatrixRolesInCombinationViolation::Basic(basic) => basic.into(),
            MatrixRolesInCombinationViolation::ForeverInvalid(
                forever_invalid,
            ) => forever_invalid.into(),
            MatrixRolesInCombinationViolation::NotYetValid(not_yet_valid) => {
                not_yet_valid.into()
            }
        }
    }
}
impl From<MatrixBuilderValidation> for CommonError {
    fn from(value: MatrixBuilderValidation) -> Self {
        match value {
            MatrixBuilderValidation::RoleInIsolation { role, violation } => {
                (role, violation).into()
            }
            MatrixBuilderValidation::CombinationViolation(violation) => {
                violation.into()
            }
        }
    }
}
