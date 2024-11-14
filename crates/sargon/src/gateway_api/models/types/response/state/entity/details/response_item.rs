use crate::prelude::*;

#[derive(
    Deserialize,
    Serialize, /* Serialize so we can test roundtrip of JSON vectors */
    Clone,
    PartialEq,
    Eq,
    Debug,
)]
pub struct StateEntityDetailsResponseItem {
    /// Bech32m-encoded human readable version of the address.
    pub address: Address,

    /// Fungible resources collection.
    pub fungible_resources: Option<FungibleResourcesCollection>,

    /// Non-fungible resources collection.
    pub non_fungible_resources: Option<NonFungibleResourcesCollection>,

    /// Entity metadata collection.
    pub metadata: EntityMetadataCollection,
}

impl StateEntityDetailsResponseItem {
    pub fn new(
        address: Address,
        fungible_resources: impl Into<Option<FungibleResourcesCollection>>,
        non_fungible_resources: impl Into<Option<NonFungibleResourcesCollection>>,
        metadata: EntityMetadataCollection,
    ) -> StateEntityDetailsResponseItem {
        StateEntityDetailsResponseItem {
            address,
            fungible_resources: fungible_resources.into(),
            non_fungible_resources: non_fungible_resources.into(),
            metadata,
        }
    }
}
