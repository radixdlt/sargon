use crate::prelude::*;

/// The platform from which a deferred link was originated.
/// More info [here](https://radixdlt.atlassian.net/wiki/spaces/PROJ/pages/3514990595/URL+Parameters+Definition+for+Wallet+RadQuest+Onboarding#Deep-Link-Value)
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DeferredDeepLinkMethod {
    Mobile,
    Desktop,
}

impl HasSampleValues for DeferredDeepLinkMethod {
    fn sample() -> Self {
        DeferredDeepLinkMethod::Mobile
    }

    fn sample_other() -> Self {
        DeferredDeepLinkMethod::Desktop
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DeferredDeepLinkMethod;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", SUT::Mobile), "Mobile");
        assert_eq!(format!("{:?}", SUT::Desktop), "Desktop");
    }

    #[test]
    fn json_roundtrip() {
        assert_json_value_eq_after_roundtrip(&SUT::Mobile, json!("mobile"));
        assert_json_value_eq_after_roundtrip(&SUT::Desktop, json!("desktop"));
    }
}
