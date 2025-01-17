use crate::prelude::*;
use sargon::SecurityShieldBuilderStatus as InternalSecurityShieldBuilderStatus;
use sargon::SecurityShieldBuilderStatusInvalidReason as InternalSecurityShieldBuilderStatusInvalidReason;

/// Represents the status of `SecurityShieldBuilder`.
/// Used for UI representation in host applications.
#[derive(Clone, Debug, PartialEq, InternalConversion, uniffi::Enum)]
pub enum SecurityShieldBuilderStatus {
    /// The selected factor sources form a strong combination
    /// in the Security Shield building process.
    Strong,

    /// The selected factor sources form a weak combination
    /// in the Security Shield building process.
    Weak {
        /// The reason why the built shield would be weak.
        reason: SecurityShieldBuilderRuleViolationReason,
    },

    /// The selected factor sources form an invalid combination
    /// in the Security Shield building process.
    /// Example: Each role must have at least one factor.
    Invalid {
        reason: SecurityShieldBuilderStatusInvalidReason,
    },
}

/// Represents the reason why the `SecurityShieldBuilder` has an invalid status.
/// This struct contains the specific reasons for each component of the security shield builder
/// being invalid. The components include:
/// - Primary role
/// - Recovery role
/// - Confirmation role
/// - Authentication signing
#[derive(Clone, Debug, PartialEq, InternalConversion, uniffi::Record)]
pub struct SecurityShieldBuilderStatusInvalidReason {
    pub is_primary_role_factor_list_empty: bool,
    pub is_recovery_role_factor_list_empty: bool,
    pub is_confirmation_role_factor_list_empty: bool,
    pub is_auth_signing_factor_missing: bool,
}
