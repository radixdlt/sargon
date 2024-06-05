use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DappDefinitions {
    #[serde(rename = "dApps")]
    pub dapp_definitions: Vec<DappDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_path: Option<String>,
}

impl HasSampleValues for DappDefinitions {
    fn sample() -> Self {
        Self {
            dapp_definitions: vec![DappDefinition::sample()],
            callback_path: Some(String::from("callback_path")),
        }
    }

    fn sample_other() -> Self {
        Self {
            dapp_definitions: vec![DappDefinition::sample_other()],
            callback_path: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappDefinitions;

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
