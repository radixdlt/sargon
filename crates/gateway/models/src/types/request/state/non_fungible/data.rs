use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateNonFungibleDataRequest {
    /// This allows for a request to be made against a historic state. If a constraint is specified,
    /// the Gateway will resolve the request against the ledger state at that time.
    /// If not specified, requests will be made with respect to the top of the committed ledger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_ledger_state: Option<LedgerStateSelector>,

    /// Bech32m-encoded human readable version of the address.
    pub resource_address: NonFungibleResourceAddress,

    /// limited to max 100 items.
    pub non_fungible_ids: Vec<NonFungibleLocalId>,
}

impl StateNonFungibleDataRequest {
    pub fn new(
        resource_address: NonFungibleResourceAddress,
        non_fungible_ids: impl IntoIterator<Item = NonFungibleLocalId>,
        at_ledger_state: impl Into<Option<LedgerStateSelector>>,
    ) -> Self {
        Self {
            at_ledger_state: at_ledger_state.into(),
            resource_address,
            non_fungible_ids: Vec::from_iter(non_fungible_ids),
        }
    }
}
