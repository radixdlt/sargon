use crate::prelude::*;
use sargon::SecurityShieldBuilderInvalidReason as InternalSecurityShieldBuilderInvalidReason;

use thiserror::Error as ThisError;

#[repr(u32)]
#[derive(
    Clone, Debug, ThisError, PartialEq, InternalConversion, uniffi::Error,
)]
pub enum SecurityShieldBuilderInvalidReason {
    #[error("Auth Signing Factor Missing")]
    MissingAuthSigningFactor,

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
