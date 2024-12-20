use crate::prelude::*;

impl StateEntityDetailsResponse {
    pub fn new(
        ledger_state: impl Into<Option<LedgerState>>,
        items: Vec<StateEntityDetailsResponseItem>,
    ) -> Self {
        Self {
            ledger_state: ledger_state.into(),
            items,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = StateEntityDetailsResponse;

    #[test] // FIXME: impl support for rest of the response atoms
    #[ignore]
    fn json_two_accounts() {
        let _ = fixture_and_json::<SUT>(include_str!(concat!(
            env!("FIXTURES_MODELS_GW"),
            "state/response_entity_details__two_accounts.json"
        )))
        .unwrap();
        // assert_json_value_eq_after_roundtrip(&sut, json) // FIXME: Once fully implemented
    }

    #[test] // FIXME: impl support for rest of the response atoms
    #[ignore]
    fn json_single_account_many_nfts_and_fungibles() {
        let _ = fixture_and_json::<SUT>(include_str!(concat!(
            env!("FIXTURES_MODELS_GW"),
            "state/response_entity_details__single_account_many_nfts_and_fungibles.json"
        )))
        .unwrap();
        // assert_json_value_eq_after_roundtrip(&sut, json) // FIXME: Once fully implemented
    }

    #[test]
    fn json_single_account_no_assets() {
        let _ = fixture_and_json::<SUT>(include_str!(concat!(
            env!("FIXTURES_MODELS_GW"),
            "state/response_entity_details__single_account_no_assets.json"
        )))
        .unwrap();
        // assert_json_value_eq_after_roundtrip(&sut, json) // FIXME: Once fully implemented
    }

    #[test]
    fn json_single_resource() {
        let _ = fixture_and_json::<SUT>(include_str!(concat!(
            env!("FIXTURES_MODELS_GW"),
            "state/response_entity_details__single_resource.json"
        )))
        .unwrap();
        // assert_json_value_eq_after_roundtrip(&sut, json) // FIXME: Once fully implemented
    }

    #[test]
    fn json_single_resource_no_metadata() {
        let _ = fixture_and_json::<SUT>(include_str!(concat!(
            env!("FIXTURES_MODELS_GW"),
            "state/response_entity_details__single_resource_no_metadata.json"
        )))
        .unwrap();
        // assert_json_value_eq_after_roundtrip(&sut, json) // FIXME: Once fully implemented
    }
}
