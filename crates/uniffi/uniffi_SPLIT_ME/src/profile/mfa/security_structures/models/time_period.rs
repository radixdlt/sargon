use crate::prelude::*;
use sargon::TimePeriod as InternalTimePeriod;

/// Time period unit expressed in days, weeks, or years.
///
/// Used to represent in the hosts UI the time period.
#[derive(
    Clone, Debug, Copy, PartialEq, Eq, Hash, InternalConversion, uniffi::Record,
)]
pub struct TimePeriod {
    /// The value of the time period.
    pub value: u16,
    /// The unit of the time period.
    pub unit: TimePeriodUnit,
}

#[uniffi::export]
pub fn new_time_period_sample() -> TimePeriod {
    InternalTimePeriod::sample().into()
}

#[uniffi::export]
pub fn new_time_period_sample_other() -> TimePeriod {
    InternalTimePeriod::sample_other().into()
}

#[uniffi::export]
pub fn new_time_period_with_days(value: u16) -> TimePeriod {
    InternalTimePeriod::with_days(value).into()
}

#[uniffi::export]
pub fn time_period_to_days(time_period: &TimePeriod) -> u16 {
    time_period.into_internal().days()
}
