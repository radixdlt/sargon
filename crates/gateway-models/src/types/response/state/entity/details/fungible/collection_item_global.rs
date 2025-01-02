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

impl FungibleResourcesCollectionItemGloballyAggregated {
    pub fn new(
        resource_address: ResourceAddress,
        amount: Decimal192,
    ) -> FungibleResourcesCollectionItemGloballyAggregated {
        FungibleResourcesCollectionItemGloballyAggregated {
            resource_address,
            amount,
        }
    }
}

impl HasSampleValues for FungibleResourcesCollectionItemGloballyAggregated {
    fn sample() -> Self {
        Self::new(ResourceAddress::sample_stokenet_xrd(), Decimal192::sample())
    }

    fn sample_other() -> Self {
        Self::new(
            ResourceAddress::sample_stokenet_candy(),
            Decimal192::sample_other(),
        )
    }
}
