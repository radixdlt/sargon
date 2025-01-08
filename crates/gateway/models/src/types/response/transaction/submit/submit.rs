use crate::prelude::*;

#[derive(
    Deserialize,
    Serialize, /* Serialize so we can test roundtrip of JSON vectors */
    Clone,
    PartialEq,
    Eq,
    Debug,
)]
pub struct TransactionSubmitResponse {
    /** Is true if the transaction is a duplicate of an existing pending transaction. */
    pub duplicate: bool,
}

#[cfg(test)]
mod tests {
    use prelude::fixture_gw_model;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionSubmitResponse;

    #[test]
    fn response_json_test() {
        let (sut, json) = fixture_and_json::<SUT>(fixture_gw_model!(
            "transaction/response_submit"
        ))
        .unwrap();
        assert_json_value_eq_after_roundtrip(&sut, json) // FIXME: Once fully implemented
    }
}
