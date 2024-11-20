use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug, EnumAsInner)]
#[serde(untagged)]
pub enum NonFungibleResourcesCollectionItem {
    Global(NonFungibleResourcesCollectionItemGloballyAggregated),
}

impl HasSampleValues for NonFungibleResourcesCollectionItem {
    fn sample() -> Self {
        Self::Global(
            NonFungibleResourcesCollectionItemGloballyAggregated::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::Global(
            NonFungibleResourcesCollectionItemGloballyAggregated::sample_other(
            ),
        )
    }
}

impl NonFungibleResourcesCollectionItem {
    pub fn resource_address(&self) -> ResourceAddress {
        match self {
            Self::Global(item) => item.resource_address,
        }
    }
}
