use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct NonFungibleResourcesCollectionItemGloballyAggregated {
    /// Bech32m-encoded human readable version of the address.
    pub resource_address: ResourceAddress,

    /// The total amount of non-fungible IDs across all vaults.
    pub amount: u64,
}

impl NonFungibleResourcesCollectionItemGloballyAggregated {
    pub fn new(
        resource_address: ResourceAddress,
        amount: u64,
    ) -> NonFungibleResourcesCollectionItemGloballyAggregated {
        NonFungibleResourcesCollectionItemGloballyAggregated {
            resource_address,
            amount,
        }
    }
}

impl HasSampleValues for NonFungibleResourcesCollectionItemGloballyAggregated {
    fn sample() -> Self {
        Self::new(ResourceAddress::sample_stokenet_nft_abandon(), 5)
    }

    fn sample_other() -> Self {
        Self::new(ResourceAddress::sample_stokenet_nft_other(), 10)
    }
}
