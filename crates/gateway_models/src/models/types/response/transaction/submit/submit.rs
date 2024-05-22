use crate::prelude::*;

#[derive(
    Deserialize,
    Serialize, /* Serialize so we can test roundtrip of JSON vectors */
    Clone,
    PartialEq,
    Eq,
    Debug,
    uniffi::Record,
)]
pub struct TransactionSubmitResponse {
    /** Is true if the transaction is a duplicate of an existing pending transaction. */
    pub duplicate: bool,
}
