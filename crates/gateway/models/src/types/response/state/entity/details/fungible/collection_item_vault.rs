use crate::prelude::*;

/// A fungible resource an entity holds, listed per vault.
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct FungibleResourcesCollectionItemVaultAggregated {
    pub resource_address: ResourceAddress,

    pub vaults: FungibleResourcesCollectionItemVaults,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct FungibleResourcesCollectionItemVaults {
    pub items: Vec<FungibleResourcesCollectionItemVaultAggregatedVaultItem>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct FungibleResourcesCollectionItemVaultAggregatedVaultItem {
    pub vault_address: VaultAddress,

    pub amount: Decimal192,
}
