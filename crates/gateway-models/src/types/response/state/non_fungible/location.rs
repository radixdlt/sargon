use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateNonFungibleLocationResponse {
    pub ledger_state: LedgerState,

    /// Bech32m-encoded human readable version of the address.
    pub resource_address: ResourceAddress,

    pub non_fungible_ids: Vec<StateNonFungibleLocationResponseItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateNonFungibleLocationResponseItem {
    /// String-encoded non-fungible ID.
    pub non_fungible_id: NonFungibleLocalId,

    pub is_burned: bool,

    /// The most recent state version underlying object was modified at.
    pub last_updated_at_state_version: i64,

    /// Bech32m-encoded human readable version of the address
    pub owning_vault_address: VaultAddress,

    /// Bech32m-encoded human readable version of the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owning_vault_parent_ancestor_address: Option<Address>,

    /// Bech32m-encoded human readable version of the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owning_vault_global_ancestor_address: Option<Address>,
}
