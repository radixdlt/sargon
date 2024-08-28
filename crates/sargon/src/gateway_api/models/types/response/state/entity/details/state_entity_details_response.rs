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
    uniffi::Record,
)]
pub struct StateEntityDetailsResponse {
    pub items: Vec<StateEntityDetailsResponseItem>,
}
