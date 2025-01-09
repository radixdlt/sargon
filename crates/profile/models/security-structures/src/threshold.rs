use crate::prelude::*;

/// A kind of threshold, either All or a specific number of factors
/// must be used to perform some function with.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum Threshold {
    /// All factors in the threshold factors list must be used to perform some function with
    All,
    /// A specific number of factors in the threshold factors list must be used to perform some function with
    Specific(u8),
}

impl Threshold {
    /// Returns the threshold value considering the number of threshold factors for `ThresholdKind::All`.
    pub fn value(&self, threshold_factor_count: usize) -> u8 {
        match self {
            Threshold::All => threshold_factor_count as u8,
            Threshold::Specific(value) => *value,
        }
    }
}

impl HasSampleValues for Threshold {
    fn sample() -> Self {
        Threshold::All
    }

    fn sample_other() -> Self {
        Threshold::Specific(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Threshold;

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
    fn json_roundtrip_all() {
        let model = SUT::All;
        assert_json_value_eq_after_roundtrip(&model, json!("all"));
        assert_json_roundtrip(&model);
    }

    #[test]
    fn json_roundtrip_specific() {
        let model = SUT::Specific(2);
        assert_json_value_eq_after_roundtrip(&model, json!({"specific": 2}));
        assert_json_roundtrip(&model);
    }
}
