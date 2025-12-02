use crate::prelude::*;
use sargon::TimePeriodUnit as InternalTimePeriodUnit;

/// A unit for the emergency fallback period.
/// Primarily used by hosts for UI representation.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum,
)]
pub enum TimePeriodUnit {
    Minutes,
    Days,
    Weeks,
}
