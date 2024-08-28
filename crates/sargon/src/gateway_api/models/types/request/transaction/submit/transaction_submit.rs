use crate::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize, /* Deserialize so we can test roundtrip of JSON vectors */
)]
pub struct TransactionSubmitRequest {
    /** Hex-encoded notarized transaction payload which can be submitted. */
    pub(crate) notarized_transaction_hex: String,
}
