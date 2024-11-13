use crate::prelude::*;

#[derive(
    Deserialize,
    Serialize,
    /* Serialize so we can test roundtrip of JSON vectors */
    Clone,
    PartialEq,
    Eq,
    Debug,
)]
pub struct FungibleResourcesCollectionItemGloballyAggregated {
    /// Bech32m-encoded human readable version of the address.
    pub resource_address: ResourceAddress,

    /// Decimal representing the amount of a related fungible resource.
    pub amount: Decimal192,
}
