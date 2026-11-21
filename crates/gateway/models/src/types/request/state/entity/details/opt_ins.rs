use metadata::prelude::MetadataKey;

use crate::prelude::*;

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize, /* Deserialize so we can test roundtrip of JSON vectors */
)]
pub struct StateEntityDetailsOptIns {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_metadata: Option<Vec<MetadataKey>>,

    /// Whether the ids of the non-fungibles in each vault are included, with
    /// resources listed per vault.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_fungible_include_nfids: Option<bool>,
}

impl StateEntityDetailsOptIns {
    pub fn new(
        explicit_metadata: Option<Vec<MetadataKey>>,
    ) -> StateEntityDetailsOptIns {
        StateEntityDetailsOptIns {
            explicit_metadata,
            non_fungible_include_nfids: None,
        }
    }
}
