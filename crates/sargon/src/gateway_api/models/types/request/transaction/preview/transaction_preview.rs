use crate::prelude::*;

#[derive(
    Serialize,
    Deserialize, /* Deserialize so we can test roundtrip of JSON vectors */
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub(crate) struct TransactionPreviewRequest {
    /** A text-representation of a transaction manifest */
    pub(crate) manifest: String,

    /** An array of hex-encoded blob data (optional) */
    pub(crate) blobs_hex: Option<Vec<String>>,

    /** An integer between `0` and `10^10`, marking the epoch at which the transaction starts being valid */
    pub(crate) start_epoch_inclusive: u64,

    /** An integer between `0` and `10^10`, marking the epoch at which the transaction is no longer valid */
    pub(crate) end_epoch_exclusive: u64,

    pub(crate) notary_public_key: Option<GWPublicKey>,

    /** Whether the notary should count as a signatory (optional, default false) */
    pub(crate) notary_is_signatory: bool,

    /** An integer between `0` and `65535`, giving the validator tip as a percentage amount. A value of `1` corresponds to 1% of the fee. */
    pub(crate) tip_percentage: u16,

    /** A decimal-string-encoded integer between `0` and `2^32 - 1`, used to ensure the transaction intent is unique. */
    pub(crate) nonce: u32,

    /** A list of public keys to be used as transaction signers */
    pub(crate) signer_public_keys: Vec<GWPublicKey>,

    pub(crate) flags: TransactionPreviewRequestFlags,
}
