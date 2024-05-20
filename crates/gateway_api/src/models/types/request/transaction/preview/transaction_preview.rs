use crate::prelude::*;

#[derive(
    Serialize,
    Deserialize, /* Deserialize so we can test roundtrip of JSON vectors */
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub struct TransactionPreviewRequest {
    /** A text-representation of a transaction manifest */
    pub manifest: String,

    /** An array of hex-encoded blob data (optional) */
    pub blobs_hex: Option<Vec<String>>,

    /** An integer between `0` and `10^10`, marking the epoch at which the transaction starts being valid */
    pub start_epoch_inclusive: u64,

    /** An integer between `0` and `10^10`, marking the epoch at which the transaction is no longer valid */
    pub end_epoch_exclusive: u64,

    pub notary_public_key: Option<GWPublicKey>,

    /** Whether the notary should count as a signatory (optional, default false) */
    pub notary_is_signatory: bool,

    /** An integer between `0` and `65535`, giving the validator tip as a percentage amount. A value of `1` corresponds to 1% of the fee. */
    pub tip_percentage: u16,

    /** A decimal-string-encoded integer between `0` and `2^32 - 1`, used to ensure the transaction intent is unique. */
    pub nonce: u32,

    /** A list of public keys to be used as transaction signers */
    pub signer_public_keys: Vec<GWPublicKey>,

    pub flags: TransactionPreviewRequestFlags,
}
