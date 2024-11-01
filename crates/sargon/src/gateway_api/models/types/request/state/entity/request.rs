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
    pub addresses: Vec<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_ins: Option<StateEntityDetailsOptIns>,
}

impl StateEntityDetailsRequest {
    pub fn new(addresses: Vec<Address>) -> StateEntityDetailsRequest {
        StateEntityDetailsRequest {
            addresses,
            opt_ins: None,
        }
    }

    pub fn address(
        address: Address,
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
