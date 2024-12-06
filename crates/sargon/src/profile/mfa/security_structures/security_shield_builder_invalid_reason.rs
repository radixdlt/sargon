use crate::prelude::*;

pub trait AsShieldBuilderViolation {
    fn as_shield_validation(
        &self,
    ) -> Option<SecurityShieldBuilderInvalidReason>;
}

impl<T: std::fmt::Debug> AsShieldBuilderViolation
    for Result<T, MatrixBuilderValidation>
{
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
            FactorSourceNotFoundInAnyRole =>
                unreachable!("Cannot happen since this error is not returned by 'validate'/'build'."),
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
        let (role_kind, violation) = self;
        match violation {
            RoleBuilderValidation::BasicViolation(basic) => unreachable!("Programmer error. Should have prevented this from happening: '{:?}'", basic),
            RoleBuilderValidation::ForeverInvalid(forever) => {
                forever.as_shield_validation()
            }
            RoleBuilderValidation::NotYetValid(not_yet_valid) => {
                (*role_kind, *not_yet_valid).as_shield_validation()
            }
        }
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
            RecoveryRoleSecurityQuestionsNotSupported => {
                SecurityShieldBuilderInvalidReason::RecoveryRoleSecurityQuestionsNotSupported
            }
            RecoveryRolePasswordNotSupported => {
                SecurityShieldBuilderInvalidReason::RecoveryRolePasswordNotSupported
            }
            ConfirmationRoleTrustedContactNotSupported => {
                SecurityShieldBuilderInvalidReason::ConfirmationRoleTrustedContactNotSupported
            }
        };
        Some(reason)
    }
}

impl SecurityShieldBuilderInvalidReason {
    pub(crate) fn role_must_have_at_least_one_factor(
        role_kind: &RoleKind,
    ) -> Self {
        match role_kind {
            RoleKind::Primary => Self::PrimaryRoleMustHaveAtLeastOneFactor,
            RoleKind::Recovery => Self::RecoveryRoleMustHaveAtLeastOneFactor,
            RoleKind::Confirmation => {
                Self::ConfirmationRoleMustHaveAtLeastOneFactor
            }
        }
    }
}

impl AsShieldBuilderViolation for (RoleKind, NotYetValidReason) {
    fn as_shield_validation(
        &self,
    ) -> Option<SecurityShieldBuilderInvalidReason> {
        let (role_kind, violation) = self;
        use NotYetValidReason::*;
        let reason = match violation {
            RoleMustHaveAtLeastOneFactor => SecurityShieldBuilderInvalidReason::role_must_have_at_least_one_factor(role_kind),
            PrimaryRoleWithPasswordInThresholdListMustHaveAnotherFactor => {
                SecurityShieldBuilderInvalidReason::PrimaryRoleWithPasswordInThresholdListMustHaveAnotherFactor
            }
            PrimaryRoleWithThresholdFactorsCannotHaveAThresholdValueOfZero => {
                SecurityShieldBuilderInvalidReason::PrimaryRoleWithThresholdFactorsCannotHaveAThresholdValueOfZero
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

// #[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]

#[repr(u32)]
#[derive(Clone, Debug, thiserror::Error, PartialEq)]
pub enum SecurityShieldBuilderInvalidReason {
    #[error("Shield name is invalid")]
    ShieldNameInvalid,

    #[error("The number of days until auto confirm must be greater than zero")]
    NumberOfDaysUntilAutoConfirmMustBeGreaterThanZero,

    #[error("Recovery and confirmation factors overlap. No factor may be used in both the recovery and confirmation roles")]
    RecoveryAndConfirmationFactorsOverlap,

    #[error("The single factor used in the primary role must not be used in any other role")]
    SingleFactorUsedInPrimaryMustNotBeUsedInAnyOtherRole,

    // =========================
    // NotYetValidReason
    // =========================
    #[error("PrimaryRole must have at least one factor")]
    PrimaryRoleMustHaveAtLeastOneFactor,

    #[error("RecoveryRole must have at least one factor")]
    RecoveryRoleMustHaveAtLeastOneFactor,

    #[error("ConfirmationRole must have at least one factor")]
    ConfirmationRoleMustHaveAtLeastOneFactor,

    #[error(
        "Primary role with password in threshold list must have another factor"
    )]
    PrimaryRoleWithPasswordInThresholdListMustHaveAnotherFactor,

    #[error(
        "Primary role with threshold factors cannot have a threshold of zero"
    )]
    PrimaryRoleWithThresholdFactorsCannotHaveAThresholdValueOfZero,

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

    #[error("Recovery role Security Questions not supported")]
    RecoveryRoleSecurityQuestionsNotSupported,

    #[error("Recovery role password not supported")]
    RecoveryRolePasswordNotSupported,

    #[error("Confirmation role cannot contain Trusted Contact")]
    ConfirmationRoleTrustedContactNotSupported,
}
