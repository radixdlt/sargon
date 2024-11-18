use crate::prelude::*;

/// The response a call to the REST Endpoint:
/// `https://mainnet.radixdlt.com/state/entity/details`
///
/// Which contains token balances of an account.
#[derive(
    Deserialize,
    Serialize, /* Serialize so we can test roundtrip of JSON vectors */
    Clone,
    PartialEq,
    Eq,
    Debug,
)]
pub struct StateEntityDetailsResponse {
    /// The ledger state against which the response was generated.
    pub ledger_state: Option<LedgerState>,

    /// The details for the requested entities.
    pub items: Vec<StateEntityDetailsResponseItem>,
}
