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

impl StateEntityDetailsResponse {
    pub fn new(
        ledger_state: impl Into<Option<LedgerState>>,
        items: impl IntoIterator<Item = StateEntityDetailsResponseItem>,
    ) -> Self {
        Self {
            ledger_state: ledger_state.into(),
            items: items.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use prelude::fixture_gw_model;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = StateEntityDetailsResponse;

    #[test] // FIXME: impl support for rest of the response atoms
    #[ignore]
    fn json_two_accounts() {
        let _ = fixture_and_json::<SUT>(fixture_gw_model!(
            "state/response_entity_details__two_accounts"
        ))
        .unwrap();
        // assert_json_value_eq_after_roundtrip(&sut, json) // FIXME: Once fully implemented
    }

    #[test] // FIXME: impl support for rest of the response atoms
    #[ignore]
    fn json_single_account_many_nfts_and_fungibles() {
        let _ = fixture_and_json::<SUT>(fixture_gw_model!(
            "state/response_entity_details__single_account_many_nfts_and_fungibles"
        ))
        .unwrap();
        // assert_json_value_eq_after_roundtrip(&sut, json) // FIXME: Once fully implemented
    }

    #[test]
    fn json_single_account_no_assets() {
        let _ = fixture_and_json::<SUT>(fixture_gw_model!(
            "state/response_entity_details__single_account_no_assets"
        ))
        .unwrap();
        // assert_json_value_eq_after_roundtrip(&sut, json) // FIXME: Once fully implemented
    }

    #[test]
    fn json_single_resource() {
        let _ = fixture_and_json::<SUT>(fixture_gw_model!(
            "state/response_entity_details__single_resource"
        ))
        .unwrap();
        // assert_json_value_eq_after_roundtrip(&sut, json) // FIXME: Once fully implemented
    }

    #[test]
    fn json_single_resource_no_metadata() {
        let _ = fixture_and_json::<SUT>(fixture_gw_model!(
            "state/response_entity_details__single_resource_no_metadata"
        ))
        .unwrap();
        // assert_json_value_eq_after_roundtrip(&sut, json) // FIXME: Once fully implemented
    }
}
