use crate::prelude::*;

/// Struct that represents content of well known file
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DappWellKnownData {
    /// List of dapp definitions
    #[serde(rename = "dApps")]
    pub dapp_definitions: Vec<DappDefinition>,
}

impl DappWellKnownData {
    pub fn new(
        dapp_definitions: impl IntoIterator<Item = DappDefinition>,
    ) -> Self {
        Self {
            dapp_definitions: dapp_definitions.into_iter().collect(),
        }
    }
}

impl HasSampleValues for DappWellKnownData {
    fn sample() -> Self {
        Self::new(vec![DappDefinition::sample()])
    }

    fn sample_other() -> Self {
        Self::new(vec![DappDefinition::sample_other()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappWellKnownData;

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
    fn request_json_test() {
        let (sut, json) = fixture_and_json::<SUT>(include_str!(concat!(
            env!("FIXTURES_MODELS"),
            "well_known.json"
        )))
        .unwrap();
        assert_json_value_eq_after_roundtrip(&sut, json)
    }
}
