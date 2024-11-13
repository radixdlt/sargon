use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug, EnumAsInner)]
#[serde(untagged)]
pub enum NonFungibleResourcesCollectionItem {
    Global(NonFungibleResourcesCollectionItemGloballyAggregated),
}
