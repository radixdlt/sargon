use crate::prelude::*;

#[derive(
    Deserialize,
    Serialize, /* Serialize so we can test roundtrip of JSON vectors */
    Clone,
    PartialEq,
    Eq,
    Debug,
)]
pub struct TransactionPreviewResponse {
    /** Hex-encoded binary blob. */
    pub encoded_receipt: String,
    pub radix_engine_toolkit_receipt:
        Option<ScryptoSerializableToolkitTransactionReceipt>,
    pub logs: Vec<TransactionPreviewResponseLogsInner>,
    pub receipt: TransactionReceipt,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionPreviewResponse;

    #[test]
    fn response_json_test() {
        let _ = fixture_and_json::<SUT>(include_str!(concat!(
            env!("FIXTURES_MODELS_GW"),
            "transaction/response_preview.json"
        )))
        .unwrap();
        // assert_json_value_eq_after_roundtrip(&sut, json) // FIXME: Once fully implemented
    }
}
