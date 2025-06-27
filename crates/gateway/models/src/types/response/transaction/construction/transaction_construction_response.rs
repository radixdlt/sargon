use crate::prelude::*;

#[derive(
    Deserialize,
    Serialize, /* Serialize so we can test roundtrip of JSON vectors */
    Clone,
    PartialEq,
    Eq,
    Debug,
)]
pub struct TransactionConstructionResponse {
    pub ledger_state: LedgerState,
}
impl TransactionConstructionResponse {
    pub fn new(ledger_state: LedgerState) -> Self {
        Self { ledger_state }
    }
}

#[cfg(test)]
mod tests {
    use prelude::fixture_gw_model;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionConstructionResponse;

    #[test]
    fn response_json_test() {
        let (sut, json) = fixture_and_json::<SUT>(fixture_gw_model!(
            "transaction/response_construction"
        ))
        .unwrap();
        assert_json_value_eq_after_roundtrip(&sut, json)
    }
}
