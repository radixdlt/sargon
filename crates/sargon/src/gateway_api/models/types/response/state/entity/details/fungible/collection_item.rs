use crate::prelude::*;

#[derive(
    Deserialize,
    Serialize, /* Serialize so we can test roundtrip of JSON vectors */
    Clone,
    PartialEq,
    Eq,
    Debug,
    EnumAsInner,
)]
#[serde(untagged)]
pub enum FungibleResourcesCollectionItem {
    Global(FungibleResourcesCollectionItemGloballyAggregated),
}

impl HasSampleValues for FungibleResourcesCollectionItem {
    fn sample() -> Self {
        Self::Global(FungibleResourcesCollectionItemGloballyAggregated::sample())
    }

    fn sample_other() -> Self {
        Self::Global(
            FungibleResourcesCollectionItemGloballyAggregated::sample_other(),
        )
    }
}
