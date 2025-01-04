use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateEntityPageNonFungibleVaultIdsRequest {
    /// Bech32m-encoded human readable version of the address.
    pub address: Address,

    /// Bech32m-encoded human readable version of the address.
    pub vault_address: VaultAddress,

    /// Bech32m-encoded human readable version of the address.
    pub resource_address: ResourceAddress,

    /// This allows for a request to be made against a historic state. If a constraint is specified,
    /// the Gateway will resolve the request against the ledger state at that time.
    /// If not specified, requests will be made with respect to the top of the committed ledger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_ledger_state: Option<LedgerStateSelector>,

    /// This cursor allows forward pagination, by providing the cursor from the previous request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// The page size requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_per_page: Option<u64>,
}

impl StateEntityPageNonFungibleVaultIdsRequest {
    pub fn new(
        address: Address,
        vault_address: VaultAddress,
        resource_address: ResourceAddress,
        at_ledger_state: impl Into<Option<LedgerStateSelector>>,
        cursor: impl Into<Option<String>>,
        limit_per_page: impl Into<Option<u64>>,
    ) -> Self {
        Self {
            address,
            vault_address,
            resource_address,
            at_ledger_state: at_ledger_state.into(),
            cursor: cursor.into(),
            limit_per_page: limit_per_page.into(),
        }
    }
}
