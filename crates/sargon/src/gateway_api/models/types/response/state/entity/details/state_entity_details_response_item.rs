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
    pub address: Address,
    pub fungible_resources: Option<FungibleResourcesCollection>,
    pub metadata: EntityMetadataCollection,
}
