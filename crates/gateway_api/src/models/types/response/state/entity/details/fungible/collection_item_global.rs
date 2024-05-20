use crate::prelude::*;

#[derive(
    Deserialize,
    Serialize,
    /* Serialize so we can test roundtrip of JSON vectors */
    Clone,
    PartialEq,
    Eq,
    Debug,
    uniffi::Record,
)]
pub struct FungibleResourcesCollectionItemGloballyAggregated {
    pub amount: Decimal192,
    pub resource_address: ResourceAddress,
}
