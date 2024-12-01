use crate::prelude::*;

impl From<BasicViolation> for CommonError {
    fn from(value: BasicViolation) -> Self {
        match value {
            BasicViolation::FactorSourceNotFound => {
                CommonError::FactorSourceNotFound
            }
            BasicViolation::RecoveryCannotSetThreshold => {
                CommonError::RecoveryCannotSetThreshold
            }
            BasicViolation::ConfirmationCannotSetThreshold => {
                CommonError::ConfirmationCannotSetThreshold
            }
        }
    }
}
impl From<ForeverInvalidReason> for CommonError {
    fn from(value: ForeverInvalidReason) -> Self {
        use ForeverInvalidReason::*;

        match value {
            FactorSourceAlreadyPresent => Self::FactorSourceAlreadyPresent,
            PrimaryCannotHaveMultipleDevices => {
                Self::PrimaryCannotHaveMultipleDevices
            }
            PrimaryCannotHavePasswordInOverrideList => {
                Self::PrimaryCannotHavePasswordInOverrideList
            }
            PrimaryCannotContainSecurityQuestions => {
                Self::PrimaryCannotContainSecurityQuestions
            }
            PrimaryCannotContainTrustedContact => {
                Self::PrimaryCannotContainTrustedContact
            }
            RecoveryRoleThresholdFactorsNotSupported => {
                Self::RecoveryRoleThresholdFactorsNotSupported
            }
            RecoveryRoleSecurityQuestionsNotSupported => {
                Self::RecoveryRoleSecurityQuestionsNotSupported
            }
            RecoveryRolePasswordNotSupported => {
                Self::RecoveryRolePasswordNotSupported
            }
            ConfirmationRoleThresholdFactorsNotSupported => {
                Self::ConfirmationRoleThresholdFactorsNotSupported
            }
            ConfirmationRoleTrustedContactNotSupported => {
                Self::ConfirmationRoleTrustedContactNotSupported
            }
        }
    }
}
impl From<NotYetValidReason> for CommonError {
    fn from(value: NotYetValidReason) -> Self {
        use NotYetValidReason::*;
        match value {
            RoleMustHaveAtLeastOneFactor => {
                Self::RoleMustHaveAtLeastOneFactor
            },
            PrimaryRoleWithPasswordInThresholdListMustHaveAnotherFactor => Self::PrimaryRoleWithPasswordInThresholdListMustHaveAnotherFactor,
            PrimaryRoleWithThresholdCannotBeZeroWithFactors => Self::PrimaryRoleWithThresholdCannotBeZeroWithFactors,
            PrimaryRoleWithPasswordInThresholdListMustThresholdGreaterThanOne => Self::PrimaryRoleWithPasswordInThresholdListMustThresholdGreaterThanOne,
            ThresholdHigherThanThresholdFactorsLen => Self::ThresholdHigherThanThresholdFactorsLen
        }
    }
}
impl From<(RoleKind, RoleBuilderValidation)> for CommonError {
    fn from(value: (RoleKind, RoleBuilderValidation)) -> Self {
        let (_role, violation) = value;
        match violation {
            RoleBuilderValidation::BasicViolation(val) => val.into(),
            RoleBuilderValidation::ForeverInvalid(val) => val.into(),
            RoleBuilderValidation::NotYetValid(val) => val.into(),
        }
    }
}
