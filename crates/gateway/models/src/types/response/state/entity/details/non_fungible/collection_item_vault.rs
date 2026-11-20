use crate::prelude::*;

/// A non-fungible resource an entity holds, listed per vault, with the ids
/// of the non-fungibles when requested.
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct NonFungibleResourcesCollectionItemVaultAggregated {
    pub resource_address: ResourceAddress,

    pub vaults: NonFungibleResourcesCollectionItemVaults,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct NonFungibleResourcesCollectionItemVaults {
    pub items: Vec<NonFungibleResourcesCollectionItemVaultAggregatedVaultItem>,
}

impl NonFungibleResourcesCollectionItemVaultAggregated {
    /// The ids of the non-fungibles in all vaults.
    pub fn non_fungible_ids(&self) -> Vec<NonFungibleLocalId> {
        self.vaults
            .items
            .iter()
            .flat_map(|vault| vault.items.clone().unwrap_or_default())
            .collect()
    }
}
