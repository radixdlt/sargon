use crate::prelude::*;
use serde::de::DeserializeOwned;

#[derive(Deserialize, Clone, PartialEq, Eq, Debug, uniffi::Record)]
pub struct FungibleResourcesCollectionItemGloballyAggregated {
    pub amount: Decimal192,
    pub resource_address: ResourceAddress,
}

#[derive(
    Deserialize, Clone, PartialEq, Eq, Debug, EnumAsInner, uniffi::Enum,
)]
#[serde(untagged)]
pub enum FungibleResourcesCollectionItem {
    Global(FungibleResourcesCollectionItemGloballyAggregated),
}

impl FungibleResourcesCollectionItem {
    pub fn resource_address(&self) -> ResourceAddress {
        match self {
            Self::Global(item) => item.resource_address,
        }
    }
}

#[derive(Deserialize, Clone, PartialEq, Eq, Debug, uniffi::Record)]
pub struct FungibleResourcesCollection {
    pub items: Vec<FungibleResourcesCollectionItem>,
}

#[derive(Deserialize, Clone, PartialEq, Eq, Debug, uniffi::Record)]
pub struct StateEntityDetailsResponseItem {
    pub address: Address,
    pub fungible_resources: Option<FungibleResourcesCollection>,
}

/// The response a call to the REST Endpoint:
/// `https://mainnet.radixdlt.com/state/entity/details`
///
/// Which contains token balances of an account.
#[derive(Deserialize, Clone, PartialEq, Eq, Debug, uniffi::Record)]
pub struct StateEntityDetailsResponse {
    pub items: Vec<StateEntityDetailsResponseItem>,
}
