use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateEntityNonFungibleResourceVaultsPageResponse {
    pub ledger_state: Option<LedgerState>,

    /// Total number of items in underlying collection, fragment of which is available in `items` collection.
    pub total_count: Option<u64>,

    /// If specified, contains a cursor to query next page of the `items` collection.
    pub next_cursor: Option<String>,

    /// Collection of fungible resources.
    pub items: Vec<NonFungibleResourcesCollectionItemVaultAggregatedVaultItem>,

    /// Bech32m-encoded human readable version of the address.
    pub address: Address,

    /// Bech32m-encoded human readable version of the address.
    pub resource_address: ResourceAddress,
}
