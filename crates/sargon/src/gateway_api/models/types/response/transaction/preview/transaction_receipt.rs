use crate::prelude::*;

#[derive(
    Deserialize,
    Serialize,
    Clone,
    PartialEq,
    Eq,
    Debug,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{status}")]
pub struct TransactionReceipt {
    pub status: TransactionReceiptStatus,
    pub error_message: Option<String>,
}
