use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DeferredDeepLinkSpecialDapp {
    RadQuest,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DeferredDeepLinkSpecialDapp;

    #[test]
    fn equality() {
        assert_eq!(SUT::RadQuest, SUT::RadQuest);
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", SUT::RadQuest), "RadQuest");
    }

    #[test]
    fn json_roundtrip() {
        assert_json_value_eq_after_roundtrip(&SUT::RadQuest, json!("radquest"));
    }
}
