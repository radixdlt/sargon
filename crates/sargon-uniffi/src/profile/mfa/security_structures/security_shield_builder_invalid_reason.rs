use crate::prelude::*;
use sargon::SecurityShieldBuilderInvalidReason as InternalSecurityShieldBuilderInvalidReason;

use thiserror::Error as ThisError;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    InternalConversion,
    ThisError,
    uniffi::Error,
)]
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
