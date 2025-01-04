use crate::prelude::*;

/// Indicates the visibility of a resource.
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
pub enum ResourceVisibility {
    Hidden,
    Visible,
}

impl Default for ResourceVisibility {
    fn default() -> Self {
        Self::Visible
    }
}

impl HasSampleValues for ResourceVisibility {
    fn sample() -> Self {
        Self::Hidden
    }

    fn sample_other() -> Self {
        Self::Visible
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ResourceVisibility;

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
        assert_eq!(ResourceVisibility::Visible, ResourceVisibility::default());
    }

    #[test]
    fn json_roundtrip() {
        assert_json_value_eq_after_roundtrip(&SUT::Hidden, json!("hidden"));
        assert_json_roundtrip(&SUT::Hidden);
    }
}
