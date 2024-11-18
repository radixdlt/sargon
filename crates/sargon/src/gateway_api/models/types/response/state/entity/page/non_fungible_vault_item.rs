use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NonFungibleResourcesCollectionItemVaultAggregatedVaultItem {
    pub total_count: Option<u64>,

    /// Bech32m-encoded human readable version of the address.
    pub vault_address: VaultAddress,

    /// The most recent state version underlying object was modified at.
    pub last_updated_at_state_version: u64,

    /// If specified, contains a cursor to query next page of the `items` collection
    pub next_cursor: Option<String>,

    /// The page of local ids in this vault.
    pub items: Option<Vec<NonFungibleLocalId>>,
}
