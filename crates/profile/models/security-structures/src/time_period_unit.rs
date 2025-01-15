use crate::prelude::*;

/// Time period unit expressed in days, weeks, or years.
///
/// Used to represent in the hosts UI the time period.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TimePeriodUnit {
    Days,
    Weeks,
    Years,
}

impl HasSampleValues for TimePeriodUnit {
    fn sample() -> Self {
        TimePeriodUnit::Days
    }

    fn sample_other() -> Self {
        TimePeriodUnit::Weeks
    }
}

impl TimePeriodUnit {
    pub fn values(&self) -> Vec<u16> {
        match self {
            TimePeriodUnit::Days => {
                (1..=MAX_RECOVERY_CONFIRMATION_FALLBACK_PERIOD_UNITS).collect()
            }
            TimePeriodUnit::Weeks => {
                (1..=MAX_RECOVERY_CONFIRMATION_FALLBACK_PERIOD_UNITS).collect()
            }
            TimePeriodUnit::Years => {
                (1..=MAX_RECOVERY_CONFIRMATION_FALLBACK_PERIOD_UNITS).collect()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TimePeriodUnit;

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
    fn values() {
        assert_eq!(
            SUT::Days.values(),
            (1..=MAX_RECOVERY_CONFIRMATION_FALLBACK_PERIOD_UNITS)
                .collect::<Vec<u16>>()
        );
        assert_eq!(
            SUT::Weeks.values(),
            (1..=MAX_RECOVERY_CONFIRMATION_FALLBACK_PERIOD_UNITS)
                .collect::<Vec<u16>>()
        );
        assert_eq!(
            SUT::Years.values(),
            (1..=MAX_RECOVERY_CONFIRMATION_FALLBACK_PERIOD_UNITS)
                .collect::<Vec<u16>>()
        );
    }
}
