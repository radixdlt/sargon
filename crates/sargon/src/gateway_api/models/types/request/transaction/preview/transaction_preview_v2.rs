use crate::prelude::*;

#[derive(
    Serialize,
    Deserialize, /* Deserialize so we can test roundtrip of JSON vectors */
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub(crate) struct TransactionPreviewRequestV2 {
    /**
     * A hex-encoded, compiled RawPreviewTransaction.
     */
    pub(crate) preview_transaction: PreviewTransaction,

    pub(crate) flags: TransactionPreviewRequestFlags,

    /** A set of flags to configure the response of the transaction preview. */
    pub(crate) opt_ins: TransactionPreviewRequestOptInsV2,
}

#[derive(
    Serialize,
    Deserialize, /* Deserialize so we can test roundtrip of JSON vectors */
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub(crate) struct PreviewTransaction {
    #[serde(rename = "type")]
    pub(crate) transaction_type: PreviewTransactionType,
    pub(crate) preview_transaction_hex: String
}

#[derive(
    Serialize,
    Deserialize, /* Deserialize so we can test roundtrip of JSON vectors */
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub(crate) enum PreviewTransactionType {
    Compiled
}