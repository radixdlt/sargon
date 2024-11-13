use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct NonFungibleResourcesCollectionItemGloballyAggregated {
    /// Bech32m-encoded human readable version of the address.
    pub resource_address: ResourceAddress,

    /// The total amount of non-fungible IDs across all vaults.
    pub amount: u64,
}
