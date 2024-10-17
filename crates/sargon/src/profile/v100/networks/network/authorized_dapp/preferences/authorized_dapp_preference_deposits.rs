use crate::prelude::*;

/// Indicates whether the Wallet should show direct deposit claims for the given Dapp.
#[derive(
    Serialize,
    Deserialize,
    FromRepr,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    enum_iterator::Sequence,
    derive_more::Display,
)]
#[serde(rename_all = "camelCase")]
pub enum AuthorizedDappPreferenceDeposits {
    Hidden,
    Visible,
}

impl Default for AuthorizedDappPreferenceDeposits {
    fn default() -> Self {
        Self::Visible
    }
}

impl HasSampleValues for AuthorizedDappPreferenceDeposits {
    fn sample() -> Self {
        Self::Visible
    }

    fn sample_other() -> Self {
        Self::Hidden
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AuthorizedDappPreferenceDeposits;

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
    fn test_default() {
        assert_eq!(SUT::Visible, SUT::default());
    }

    #[test]
    fn json_roundtrip() {
        assert_json_value_eq_after_roundtrip(&SUT::Visible, json!("visible"));
        assert_json_roundtrip(&SUT::Visible);
    }
}
