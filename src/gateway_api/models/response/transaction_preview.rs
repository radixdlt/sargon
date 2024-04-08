use crate::prelude::*;

#[derive(Deserialize, Clone, PartialEq, Eq, Debug, uniffi::Record)]
pub struct TransactionPreviewResponse {
    /** Hex-encoded binary blob. */
    pub encoded_receipt: String,
    pub logs: Vec<TransactionPreviewResponseLogsInner>,
}

#[derive(
    Deserialize,
    Clone,
    PartialEq,
    Eq,
    Debug,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{message}")]
pub struct TransactionPreviewResponseLogsInner {
    pub level: String,
    pub message: String,
}
