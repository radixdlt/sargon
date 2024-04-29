use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct WalletInteractionRequests(Vec<BagOfBytes>);

impl WalletInteractionRequests {
    pub fn new(requests: impl Into<Vec<BagOfBytes>>) -> Self {
        Self(requests.into())
    }
}

impl HasSampleValues for WalletInteractionRequests {
    fn sample() -> Self {
        Self::new(vec![BagOfBytes::sample()])
    }

    fn sample_other() -> Self {
        Self::new(vec![BagOfBytes::sample_other()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = WalletInteractionRequests;

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
        let original = SUT::sample();
        let json = r#"["deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead"]"#;

        assert_eq_after_json_roundtrip(&original, json);
    }
}
