use crate::prelude::*;

#[derive(
    Deserialize,
    Serialize, /* Serialize so we can test roundtrip of JSON vectors */
    Clone,
    PartialEq,
    Eq,
    Debug,
    EnumAsInner,
    uniffi::Enum,
)]
#[serde(untagged)]
pub enum FungibleResourcesCollectionItem {
    Global(FungibleResourcesCollectionItemGloballyAggregated),
}

