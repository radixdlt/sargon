use crate::prelude::*;

/// Time period expressed by a number of `TimePeriodUnit`.
///
/// Used to represent in the hosts UI the time period until the recovery role is auto confirmed
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct TimePeriod {
    pub value: u16,
    pub unit: TimePeriodUnit,
}

impl TimePeriod {
    pub fn with_days(value: u16) -> Self {
        if (value % DAYS_PER_YEAR) == 0 {
            Self {
                value: value / DAYS_PER_YEAR,
                unit: TimePeriodUnit::Years,
            }
        } else if (value % DAYS_PER_WEEK) == 0 {
            Self {
                value: value / DAYS_PER_WEEK,
                unit: TimePeriodUnit::Weeks,
            }
        } else {
            Self {
                value,
                unit: TimePeriodUnit::Days,
            }
        }
    }

    pub fn days(&self) -> u16 {
        match self.unit {
            TimePeriodUnit::Days => self.value,
            TimePeriodUnit::Weeks => self.value * DAYS_PER_WEEK,
            TimePeriodUnit::Years => self.value * DAYS_PER_YEAR,
        }
    }
}

impl HasSampleValues for TimePeriod {
    fn sample() -> Self {
        Self {
            value: 1,
            unit: TimePeriodUnit::Days,
        }
    }

    fn sample_other() -> Self {
        Self {
            value: 1,
            unit: TimePeriodUnit::Weeks,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TimePeriod;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn days_conversion() {
        let mut sut = SUT::with_days(DAYS_PER_YEAR);
        assert_eq!(sut.days(), DAYS_PER_YEAR);
        assert_eq!(sut.value, 1);
        assert_eq!(sut.unit, TimePeriodUnit::Years);

        sut = SUT::with_days(1095);
        assert_eq!(sut.days(), 1095);
        assert_eq!(sut.value, 3);
        assert_eq!(sut.unit, TimePeriodUnit::Years);

        sut = SUT::with_days(DAYS_PER_WEEK);
        assert_eq!(sut.days(), DAYS_PER_WEEK);
        assert_eq!(sut.value, 1);
        assert_eq!(sut.unit, TimePeriodUnit::Weeks);

        sut = SUT::with_days(14);
        assert_eq!(sut.days(), 14);
        assert_eq!(sut.value, 2);
        assert_eq!(sut.unit, TimePeriodUnit::Weeks);

        sut = SUT::with_days(400);
        assert_eq!(sut.days(), 400);
        assert_eq!(sut.unit, TimePeriodUnit::Days);
    }
}
