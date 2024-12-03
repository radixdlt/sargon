use crate::prelude::*;


pub trait AsShieldBuilderViolation {
    fn as_shield_validation(
        &self,
    ) -> Option<SecurityShieldBuilderInvalidReason>;
}

impl<T> AsShieldBuilderViolation for Result<T, MatrixBuilderValidation> {
    fn as_shield_validation(
        &self,
    ) -> Option<SecurityShieldBuilderInvalidReason> {
        match self {
            Result::Err(err) => err.as_shield_validation(),
            Result::Ok(_) => None,
        }
    }
}
impl AsShieldBuilderViolation for MatrixBuilderValidation {
    fn as_shield_validation(
        &self,
    ) -> Option<SecurityShieldBuilderInvalidReason> {
        match self {
            MatrixBuilderValidation::RoleInIsolation { role, violation } => {
                (*role, *violation).as_shield_validation()
            }
            MatrixBuilderValidation::CombinationViolation(violation) => {
                violation.as_shield_validation()
            }
        }
    }
}

impl AsShieldBuilderViolation for MatrixRolesInCombinationViolation {
    fn as_shield_validation(
        &self,
    ) -> Option<SecurityShieldBuilderInvalidReason> {
        match self {
            Self::Basic(val) => val.as_shield_validation(),
            Self::ForeverInvalid(val) => val.as_shield_validation(),
            Self::NotYetValid(val) => val.as_shield_validation(),
        }
    }
}

impl AsShieldBuilderViolation for MatrixRolesInCombinationBasicViolation {
    fn as_shield_validation(
        &self,
    ) -> Option<SecurityShieldBuilderInvalidReason> {
        use MatrixRolesInCombinationBasicViolation::*;
        match self {
            FactorSourceNotFoundInAnyRole => {
                Some(SecurityShieldBuilderInvalidReason::FactorSourceNotFoundInAnyRole)
            }
            NumberOfDaysUntilAutoConfirmMustBeGreaterThanZero => {
                Some(SecurityShieldBuilderInvalidReason::NumberOfDaysUntilAutoConfirmMustBeGreaterThanZero)
            }
        }
    }
}
impl AsShieldBuilderViolation for MatrixRolesInCombinationForeverInvalid {
    fn as_shield_validation(
        &self,
    ) -> Option<SecurityShieldBuilderInvalidReason> {
        use MatrixRolesInCombinationForeverInvalid::*;
        match self {
            RecoveryAndConfirmationFactorsOverlap => {
                Some(SecurityShieldBuilderInvalidReason::RecoveryAndConfirmationFactorsOverlap)
            }
        }
    }
}
impl AsShieldBuilderViolation for MatrixRolesInCombinationNotYetValid {
    fn as_shield_validation(
        &self,
    ) -> Option<SecurityShieldBuilderInvalidReason> {
        use MatrixRolesInCombinationNotYetValid::*;

        match self {
            SingleFactorUsedInPrimaryMustNotBeUsedInAnyOtherRole => {
                Some(SecurityShieldBuilderInvalidReason::SingleFactorUsedInPrimaryMustNotBeUsedInAnyOtherRole)
            }
        }
    }
}

impl AsShieldBuilderViolation for (RoleKind, RoleBuilderValidation) {
    fn as_shield_validation(
        &self,
    ) -> Option<SecurityShieldBuilderInvalidReason> {
        let (_, violation) = self;
        match violation {
            RoleBuilderValidation::BasicViolation(basic) => {
                basic.as_shield_validation()
            }
            RoleBuilderValidation::ForeverInvalid(forever) => {
                forever.as_shield_validation()
            }
            RoleBuilderValidation::NotYetValid(not_yet_valid) => {
                not_yet_valid.as_shield_validation()
            }
        }
    }
}

impl AsShieldBuilderViolation for BasicViolation {
    fn as_shield_validation(
        &self,
    ) -> Option<SecurityShieldBuilderInvalidReason> {
        use BasicViolation::*;
        let reason = match self {
            FactorSourceNotFound => SecurityShieldBuilderInvalidReason::FactorSourceNotFound,
            RecoveryCannotSetThreshold => {
                SecurityShieldBuilderInvalidReason::RecoveryCannotSetThreshold
            }
            ConfirmationCannotSetThreshold => {
                SecurityShieldBuilderInvalidReason::ConfirmationCannotSetThreshold
            }
        };
        Some(reason)
    }
}

impl AsShieldBuilderViolation for ForeverInvalidReason {
    fn as_shield_validation(
        &self,
    ) -> Option<SecurityShieldBuilderInvalidReason> {
        use ForeverInvalidReason::*;
        let reason = match self {
            FactorSourceAlreadyPresent => SecurityShieldBuilderInvalidReason::FactorSourceAlreadyPresent,
            PrimaryCannotHaveMultipleDevices => {
                SecurityShieldBuilderInvalidReason::PrimaryCannotHaveMultipleDevices
            }
            PrimaryCannotHavePasswordInOverrideList => {
                SecurityShieldBuilderInvalidReason::PrimaryCannotHavePasswordInOverrideList
            }
            PrimaryCannotContainSecurityQuestions => {
                SecurityShieldBuilderInvalidReason::PrimaryCannotContainSecurityQuestions
            }
            PrimaryCannotContainTrustedContact => {
                SecurityShieldBuilderInvalidReason::PrimaryCannotContainTrustedContact
            }
            RecoveryRoleThresholdFactorsNotSupported => {
                SecurityShieldBuilderInvalidReason::RecoveryRoleThresholdFactorsNotSupported
            }
            RecoveryRoleSecurityQuestionsNotSupported => {
                SecurityShieldBuilderInvalidReason::RecoveryRoleSecurityQuestionsNotSupported
            }
            RecoveryRolePasswordNotSupported => {
                SecurityShieldBuilderInvalidReason::RecoveryRolePasswordNotSupported
            }
            ConfirmationRoleThresholdFactorsNotSupported => {
                SecurityShieldBuilderInvalidReason::ConfirmationRoleThresholdFactorsNotSupported
            }
            ConfirmationRoleTrustedContactNotSupported => {
                SecurityShieldBuilderInvalidReason::ConfirmationRoleTrustedContactNotSupported
            }
        };
        Some(reason)
    }
}

impl AsShieldBuilderViolation for NotYetValidReason {
    fn as_shield_validation(
        &self,
    ) -> Option<SecurityShieldBuilderInvalidReason> {
        use NotYetValidReason::*;
        let reason = match self {
            RoleMustHaveAtLeastOneFactor => SecurityShieldBuilderInvalidReason::RoleMustHaveAtLeastOneFactor,
            PrimaryRoleWithPasswordInThresholdListMustHaveAnotherFactor => {
                SecurityShieldBuilderInvalidReason::PrimaryRoleWithPasswordInThresholdListMustHaveAnotherFactor
            }
            PrimaryRoleWithThresholdCannotBeZeroWithFactors => {
                SecurityShieldBuilderInvalidReason::PrimaryRoleWithThresholdCannotBeZeroWithFactors
            }
            PrimaryRoleWithPasswordInThresholdListMustThresholdGreaterThanOne => {
                SecurityShieldBuilderInvalidReason::PrimaryRoleWithPasswordInThresholdListMustThresholdGreaterThanOne
            }
            ThresholdHigherThanThresholdFactorsLen => {
                SecurityShieldBuilderInvalidReason::ThresholdHigherThanThresholdFactorsLen
            }
        };
        Some(reason)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SecurityShieldBuilderInvalidReason {
    #[error("Shield name is invalid")]
    ShieldNameInvalid,

    #[error("The factor source was not found in any role")]
    FactorSourceNotFoundInAnyRole,

    #[error("The number of days until auto confirm must be greater than zero")]
    NumberOfDaysUntilAutoConfirmMustBeGreaterThanZero,

    #[error("Recovery and confirmation factors overlap. No factor may be used in both the recovery and confirmation roles")]
    RecoveryAndConfirmationFactorsOverlap,

    #[error("The single factor used in the primary role must not be used in any other role")]
    SingleFactorUsedInPrimaryMustNotBeUsedInAnyOtherRole,

    // ==================
    // BasicViolation
    // ==================
    /// e.g. tried to remove a factor source which was not found.
    #[error("FactorSourceID not found")]
    FactorSourceNotFound,

    #[error("Recovery cannot set threshold")]
    RecoveryCannotSetThreshold,

    #[error("Confirmation cannot set threshold")]
    ConfirmationCannotSetThreshold,

    // =========================
    // NotYetValidReason
    // =========================
    #[error("Role must have at least one factor")]
    RoleMustHaveAtLeastOneFactor,

    #[error(
        "Primary role with password in threshold list must have another factor"
    )]
    PrimaryRoleWithPasswordInThresholdListMustHaveAnotherFactor,

    #[error(
        "Primary role with threshold factors cannot have a threshold of zero"
    )]
    PrimaryRoleWithThresholdCannotBeZeroWithFactors,

    #[error("Primary role with password in threshold list must have threshold greater than one")]
    PrimaryRoleWithPasswordInThresholdListMustThresholdGreaterThanOne,

    #[error("Threshold higher than threshold factors len")]
    ThresholdHigherThanThresholdFactorsLen,

    // ================================
    // ForeverInvalidReason
    // ================================
    #[error("Factor source already present")]
    FactorSourceAlreadyPresent,

    #[error("Primary role cannot have multiple devices")]
    PrimaryCannotHaveMultipleDevices,

    #[error("Primary role cannot have password in override list")]
    PrimaryCannotHavePasswordInOverrideList,

    #[error("Primary role cannot contain Security Questions")]
    PrimaryCannotContainSecurityQuestions,

    #[error("Primary role cannot contain Trusted Contact")]
    PrimaryCannotContainTrustedContact,

    #[error("Recovery role threshold list not supported")]
    RecoveryRoleThresholdFactorsNotSupported,

    #[error("Recovery role Security Questions not supported")]
    RecoveryRoleSecurityQuestionsNotSupported,

    #[error("Recovery role password not supported")]
    RecoveryRolePasswordNotSupported,

    #[error("Confirmation role threshold list not supported")]
    ConfirmationRoleThresholdFactorsNotSupported,

    #[error("Confirmation role cannot contain Trusted Contact")]
    ConfirmationRoleTrustedContactNotSupported,
}
