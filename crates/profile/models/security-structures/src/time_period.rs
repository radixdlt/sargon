use crate::prelude::*;

/// Time period expressed by a number of `TimePeriodUnit`.
///
/// Used to represent in the hosts UI the time period until recovery can be confirmed with timed based confirmation
#[derive(
    PartialEq, Eq, Clone, Copy, Debug, StdHash, Serialize, Deserialize,
)]
pub struct TimePeriod {
    pub value: u16,
    pub unit: TimePeriodUnit,
}

impl TimePeriod {
    pub fn is_zero(&self) -> bool {
        self.value == 0
    }

    pub fn in_minutes(&self) -> u32 {
        match self.unit {
            TimePeriodUnit::Minutes => self.value as u32,
            TimePeriodUnit::Days => self.value as u32 * MINUTES_PER_DAY,
            TimePeriodUnit::Weeks => {
                self.value as u32 * DAYS_PER_WEEK as u32 * MINUTES_PER_DAY
            }
        }
    }

    pub const fn with_days(value: u16) -> Self {
        if (value % DAYS_PER_WEEK) == 0 {
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

    pub const fn with_minutes(value: u16) -> Self {
        if (value % DAYS_PER_WEEK) == 0 {
            Self {
                value: value / DAYS_PER_WEEK,
                unit: TimePeriodUnit::Weeks,
            }
        } else if (value % MINUTES_PER_DAY as u16) == 0 {
            Self {
                value,
                unit: TimePeriodUnit::Days,
            }
        } else {
            Self {
                value,
                unit: TimePeriodUnit::Minutes,
            }
        }
    }

    pub fn days(&self) -> u16 {
        match self.unit {
            TimePeriodUnit::Days => self.value,
            TimePeriodUnit::Weeks => self.value * DAYS_PER_WEEK,
            TimePeriodUnit::Minutes => panic!("Cannot convert minutes to days"),
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
    fn is_zero() {
        assert!(SUT::with_days(0).is_zero());
    }

    #[test]
    fn day_in_minutes() {
        assert_eq!(SUT::sample().in_minutes(), 1440);
    }

    #[test]
    fn week_in_minutes() {
        assert_eq!(SUT::sample_other().in_minutes(), 10080);
    }

    #[test]
    fn days_conversion() {
        let mut sut = SUT::with_days(DAYS_PER_WEEK);
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

    #[test]
    fn json_roundtrip_days() {
        assert_eq_after_json_roundtrip(
            &SUT::sample(),
            r#"
            {
                "value": 1,
                "unit": "days"
            }
            "#,
        );
    }

    #[test]
    fn json_roundtrip_weeks() {
        assert_eq_after_json_roundtrip(
            &SUT::sample_other(),
            r#"
            {
                "value": 1,
                "unit": "weeks"
            }
            "#,
        );
    }
}
