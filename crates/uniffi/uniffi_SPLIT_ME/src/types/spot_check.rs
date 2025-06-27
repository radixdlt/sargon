use crate::prelude::*;
use sargon::SpotCheckResponse as InternalSpotCheckResponse;

/// An enum indicating the result of a spot check.
///
/// Note that there isn't a failure case since user never fails a spot check. It either:
/// - succeeds (`Valid` returned by host),
/// - skips (`Skipped` returned by host),
/// - aborts (`CommonError::HostInteractionAborted` thrown by host)
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum SpotCheckResponse {
    /// The factor source was successfully validated.
    Valid,

    /// The user skipped the spot check.
    Skipped,
}
