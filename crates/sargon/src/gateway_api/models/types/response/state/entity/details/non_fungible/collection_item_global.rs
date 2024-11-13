use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct NonFungibleResourcesCollectionItemGloballyAggregated {
    /// Bech32m-encoded human readable version of the address.
    pub resource_address: ResourceAddress,

    /// The total amount of non-fungible IDs across all vaults.
    pub amount: u64,
}

impl HasSampleValues for NonFungibleResourcesCollectionItemGloballyAggregated {
    fn sample() -> Self {
        Self {
            resource_address: ResourceAddress::sample_stokenet_nft_abandon(),
            amount: 5,
        }
    }

    fn sample_other() -> Self {
        Self {
            resource_address: ResourceAddress::sample_stokenet_nft_other(),
            amount: 10,
        }
    }
}