use crate::prelude::*;

/// A quantifier of a quantity, either `atLeast` or `exactly`, as in
/// "I want AT LEAST 3" or "I want EXACTLY 10".
///
/// This is typically sent by a Dapp when requesting access to accounts
/// or PersonaData.
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    strum::Display,
)]
#[serde(rename_all = "camelCase")]
pub enum RequestedNumberQuantifier {
    /// (Request access to) *exactly* N many of something, where quantity `N` is
    /// not part of this enum, e.g. "I want EXACTLY 2 accounts"
    Exactly,

    /// (Request access to) *at least* N many of something, where quantity `N` is
    /// not part of this enum, e.g. "I want AT LEAST 3 accounts"
    AtLeast,
}

impl HasSampleValues for RequestedNumberQuantifier {
    fn sample() -> Self {
        RequestedNumberQuantifier::Exactly
    }

    fn sample_other() -> Self {
        RequestedNumberQuantifier::AtLeast
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", RequestedNumberQuantifier::Exactly),
            "Exactly"
        );
        assert_eq!(
            format!("{}", RequestedNumberQuantifier::AtLeast),
            "AtLeast"
        );
    }

    #[test]
    fn json_roundtrip() {
        assert_json_roundtrip(&RequestedNumberQuantifier::Exactly);
        assert_json_roundtrip(&RequestedNumberQuantifier::AtLeast);
        assert_json_value_eq_after_roundtrip(
            &RequestedNumberQuantifier::Exactly,
            json!("exactly"),
        );
        assert_json_value_eq_after_roundtrip(
            &RequestedNumberQuantifier::AtLeast,
            json!("atLeast"),
        );
    }
}
