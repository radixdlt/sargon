use crate::prelude::*;
use sargon::SecurityShieldPrerequisitesStatus as InternalSecurityShieldPrerequisitesStatus;

/// An enum representing the status of the prerequisites for building a Security Shield.
/// This is, whether the user has the necessary factor sources to build a Security Shield.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, InternalConversion, uniffi::Enum,
)]
pub enum SecurityShieldPrerequisitesStatus {
    /// A Security Shield can be built with the current Factor Sources available.
    Sufficient,

    /// At least one hardware Factor Source must be added in order to build a Shield.
    /// Note: this doesn't mean that after adding a hardware Factor Source we would have `Sufficient` status.
    HardwareRequired,

    /// One more Factor Source, of any category, must be added in order to build a Shield.
    AnyRequired,
}
