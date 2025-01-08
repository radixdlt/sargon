use crate::prelude::*;

/// Represents the status of selected threshold factor sources in the Security Shield building process.
/// Primarily used for UI logic representation in host applications.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectedPrimaryThresholdFactorsStatus {
    /// The selected factor sources are optimal
    /// in the Security Shield building process.
    Optimal,

    /// The selected factor sources are suboptimal
    /// in the Security Shield building process.
    ///
    /// Note: Typically used in hosts as a warning message.
    Suboptimal,

    /// The selected factor sources are insufficient
    /// in the Security Shield building process.
    Insufficient,

    /// The selected factor sources form an invalid combination
    /// in the Security Shield building process.
    ///
    /// Example: A Password factor source cannot be used alone.
    Invalid,
}
