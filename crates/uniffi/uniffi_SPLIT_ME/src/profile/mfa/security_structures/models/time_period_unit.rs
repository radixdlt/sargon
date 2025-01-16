use crate::prelude::*;
use sargon::TimePeriodUnit as InternalTimePeriodUnit;

/// A unit for the emergency fallback period.
/// Primarily used by hosts for UI representation.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum,
)]
pub enum TimePeriodUnit {
    Days,
    Weeks,
    Years,
}

#[uniffi::export]
pub fn time_period_unit_values(time_period_unit: &TimePeriodUnit) -> Vec<u16> {
    time_period_unit
        .into_internal()
        .values()
        .into_iter()
        .collect()
}
