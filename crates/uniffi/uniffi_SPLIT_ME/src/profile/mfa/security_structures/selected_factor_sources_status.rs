use crate::prelude::*;
use sargon::SelectedFactorSourcesForRoleStatus as InternalSelectedFactorSourcesForRoleStatus;

/// Represents the status of selected factor sources for a specific role in the Security Shield building process.
/// Primarily used for UI logic representation in host applications.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, InternalConversion, uniffi::Enum,
)]
pub enum SelectedFactorSourcesForRoleStatus {
    /// The selected factor sources are optimal for the specified role
    /// in the Security Shield building process.
    Optimal,

    /// The selected factor sources are suboptimal for the specified role
    /// in the Security Shield building process.
    ///
    /// Note: Typically used in hosts as a warning message.
    Suboptimal,

    /// The selected factor sources are insufficient for the specified role
    /// in the Security Shield building process.
    Insufficient,

    /// The selected factor sources form an invalid combination for the specified role
    /// in the Security Shield building process.
    ///
    /// Example: A Password factor source cannot be used alone for the Primary role.
    Invalid,
}
