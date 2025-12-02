use crate::prelude::*;

/// Time period unit expressed in days, weeks, or years.
///
/// Used to represent in the hosts UI the time period.
#[derive(
    PartialEq, Eq, Clone, Copy, Debug, StdHash, Serialize, Deserialize,
)]
pub enum TimePeriodUnit {
    #[serde(rename = "minutes")]
    Minutes,
    #[serde(rename = "days")]
    Days,
    #[serde(rename = "weeks")]
    Weeks,
}

impl HasSampleValues for TimePeriodUnit {
    fn sample() -> Self {
        TimePeriodUnit::Days
    }

    fn sample_other() -> Self {
        TimePeriodUnit::Weeks
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
    fn json_roundtrip() {
        assert_json_value_eq_after_roundtrip(&SUT::Days, json!("days"));
        assert_json_value_eq_after_roundtrip(&SUT::Weeks, json!("weeks"));
    }
}
