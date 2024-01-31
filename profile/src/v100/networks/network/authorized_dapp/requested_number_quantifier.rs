use crate::prelude::*;

/// A quantifier of a quantity, either `atLeast` or `exactly`, as in
/// "I want AT LEAST 3" or "I want EXACTLY 10".
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    strum_macros::Display,
    uniffi::Enum,
)]
#[serde(rename_all = "camelCase")]
pub enum RequestedNumberQuantifier {
    Exactly,
    AtLeast,
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
