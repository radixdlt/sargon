use crate::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = StateEntityDetailsRequest;

    #[test]
    fn json_request_entity_details_single_account_no_assets() {
        let _ = fixture_and_json::<SUT>(include_str!(concat!(
            env!("FIXTURES_MODELS_GW"),
            "state/request_entity_details__single_account_no_assets.json"
        )))
        .unwrap();
        // assert_json_value_eq_after_roundtrip(&sut, json) // FIXME: Once fully implemented
    }

    #[test]
    fn json_request_entity_details_single_resource() {
        let _ = fixture_and_json::<SUT>(include_str!(concat!(
            env!("FIXTURES_MODELS_GW"),
            "state/request_entity_details__single_resource.json"
        )))
        .unwrap();
        // assert_json_value_eq_after_roundtrip(&sut, json) // FIXME: Once fully implemented
    }

    #[test]
    fn json_request_entity_details_two_accounts() {
        let _ = fixture_and_json::<SUT>(include_str!(concat!(
            env!("FIXTURES_MODELS_GW"),
            "state/request_entity_details__two_accounts.json"
        )))
        .unwrap();
        // assert_json_value_eq_after_roundtrip(&sut, json) // FIXME: Once fully implemented
    }
}
