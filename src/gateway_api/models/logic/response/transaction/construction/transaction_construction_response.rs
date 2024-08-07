use crate::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionConstructionResponse;

    #[test]
    fn response_json_test() {
        let (sut, json) = fixture_and_json::<SUT>(include_str!(concat!(
            env!("FIXTURES_MODELS_GW"),
            "transaction/response_construction.json"
        )))
        .unwrap();
        assert_json_value_eq_after_roundtrip(&sut, json)
    }
}
