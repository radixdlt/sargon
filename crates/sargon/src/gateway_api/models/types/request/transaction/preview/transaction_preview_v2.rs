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
    pub(crate) preview_transaction: PreviewTransactionV2,

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
pub(crate) struct PreviewTransactionV2 {
    #[serde(rename = "type")]
    pub(crate) transaction_type: PreviewTransactionTypeV2,
    pub(crate) preview_transaction_hex: String,
}

#[derive(
    Serialize,
    Deserialize, /* Deserialize so we can test roundtrip of JSON vectors */
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub(crate) enum PreviewTransactionTypeV2 {
    Compiled,
}
