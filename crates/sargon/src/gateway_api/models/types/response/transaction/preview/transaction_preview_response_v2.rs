use crate::prelude::*;

#[derive(
    Deserialize,
    Serialize, /* Serialize so we can test roundtrip of JSON vectors */
    Clone,
    PartialEq,
    Eq,
    Debug,
)]
pub struct TransactionPreviewResponseV2 {
    /**
     * A summarized state of the ledger on top of which the preview was performed.
     */
    pub at_ledger_state_version: u64,

    /**
     * This is provided unless the core_api_receipt flag is set to false in the opt_ins property of the request.
     * 
     * This type is defined in the Core API as TransactionReceipt. See the Core API documentation for more details.
     */
    pub receipt: Option<TransactionReceipt>,

    /**
     * An optional field which is only provided if the radix_engine_toolkit_receipt flag is set to true in the opt_ins property of the request.
     */
    pub radix_engine_toolkit_receipt: Option<ScryptoSerializableToolkitTransactionReceipt>,

    /**
     * An optional field which is only provided if the logs flag is set to true in the opt_ins property of the request.
     */
    pub logs: Option<Vec<TransactionPreviewResponseLogsInner>>,
}
