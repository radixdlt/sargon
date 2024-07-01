use crate::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize, /* Deserialize so we can test roundtrip of JSON vectors */
)]
pub struct StateEntityDetailsRequest {
    pub(crate) addresses: Vec<String>,
    #[serde(rename = "opt_ins", skip_serializing_if = "Option::is_none")]
    pub(crate) opt_ins: Option<StateEntityDetailsOptIns>,
}

impl StateEntityDetailsRequest {
    pub fn new(addresses: Vec<String>) -> StateEntityDetailsRequest {
        StateEntityDetailsRequest {
            addresses,
            opt_ins: None,
        }
    }

    pub fn address(
        address: String,
        explicit_metadata: Vec<MetadataKey>,
    ) -> StateEntityDetailsRequest {
        StateEntityDetailsRequest {
            addresses: vec![address],
            opt_ins: Some(StateEntityDetailsOptIns::new(Some(
                explicit_metadata,
            ))),
        }
    }
}
