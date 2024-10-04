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
