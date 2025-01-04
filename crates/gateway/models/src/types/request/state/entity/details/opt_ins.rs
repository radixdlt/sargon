use metadata::prelude::MetadataKey;

use crate::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize, /* Deserialize so we can test roundtrip of JSON vectors */
)]
pub struct StateEntityDetailsOptIns {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_metadata: Option<Vec<MetadataKey>>,
}

impl StateEntityDetailsOptIns {
    pub fn new(
        explicit_metadata: Option<Vec<MetadataKey>>,
    ) -> StateEntityDetailsOptIns {
        StateEntityDetailsOptIns { explicit_metadata }
    }
}
