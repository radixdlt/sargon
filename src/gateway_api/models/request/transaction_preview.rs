use crate::prelude::*;

impl TransactionPreviewRequest {
    pub fn new(
        intent: TransactionIntent,
        signer_public_keys: impl IntoIterator<Item = PublicKey>,
        flags: impl Into<Option<TransactionPreviewRequestFlags>>,
    ) -> Self {
        Self {
            manifest: intent.manifest_string(),
            blobs_hex: Some(
                intent
                    .blobs()
                    .blobs()
                    .into_iter()
                    .map(|b| b.to_hex())
                    .collect_vec(),
            ),
            start_epoch_inclusive: intent.header.start_epoch_inclusive.into(),
            end_epoch_exclusive: intent.header.end_epoch_exclusive.into(),
            notary_public_key: Some(GWPublicKey::from(
                intent.header.notary_public_key,
            )),
            notary_is_signatory: intent.header.notary_is_signatory,
            tip_percentage: intent.header.tip_percentage,
            nonce: intent.header.nonce.into(),
            signer_public_keys: signer_public_keys
                .into_iter()
                .map(GWPublicKey::from)
                .collect_vec(),
            flags: flags
                .into()
                .unwrap_or(TransactionPreviewRequestFlags::default()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct TransactionPreviewRequest {
    /** A text-representation of a transaction manifest */
    manifest: String,

    /** An array of hex-encoded blob data (optional) */
    blobs_hex: Option<Vec<String>>,

    /** An integer between `0` and `10^10`, marking the epoch at which the transaction starts being valid */
    start_epoch_inclusive: u64,

    /** An integer between `0` and `10^10`, marking the epoch at which the transaction is no longer valid */
    end_epoch_exclusive: u64,

    notary_public_key: Option<GWPublicKey>,

    /** Whether the notary should count as a signatory (optional, default false) */
    notary_is_signatory: bool,

    /** An integer between `0` and `65535`, giving the validator tip as a percentage amount. A value of `1` corresponds to 1% of the fee. */
    tip_percentage: u16,

    /** A decimal-string-encoded integer between `0` and `2^32 - 1`, used to ensure the transaction intent is unique. */
    nonce: u32,

    /** A list of public keys to be used as transaction signers */
    signer_public_keys: Vec<GWPublicKey>,

    flags: TransactionPreviewRequestFlags,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct TransactionPreviewRequestFlags {
    use_free_credit: bool,
    assume_all_signature_proofs: bool,
    skip_epoch_check: bool,
}
impl Default for TransactionPreviewRequestFlags {
    fn default() -> Self {
        Self {
            use_free_credit: true,
            assume_all_signature_proofs: false,
            skip_epoch_check: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "key_type")]
pub(crate) enum GWPublicKey {
    Secp256k1(GWSecp256k1PublicKey),
    Ed25519(GWEd25519PublicKey),
}

impl From<PublicKey> for GWPublicKey {
    fn from(value: PublicKey) -> Self {
        match value {
            PublicKey::Ed25519 { value } => Self::Ed25519(GWEd25519PublicKey {
                key_type: GWPublicKeyType::Ed25519,
                key_hex: value.to_hex(),
            }),
            PublicKey::Secp256k1 { value } => {
                Self::Secp256k1(GWSecp256k1PublicKey {
                    key_type: GWPublicKeyType::Secp256k1,
                    key_hex: value.to_hex(),
                })
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct GWEd25519PublicKey {
    key_type: GWPublicKeyType,

    /** The hex-encoded compressed EdDSA Ed25519 public key (32 bytes) */
    key_hex: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct GWSecp256k1PublicKey {
    key_type: GWPublicKeyType,

    /** The hex-encoded compressed ECDSA Secp256k1 public key (33 bytes) */
    key_hex: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) enum GWPublicKeyType {
    #[serde(rename = "EcdsaSecp256k1")]
    Secp256k1,
    #[serde(rename = "EddsaEd25519")]
    Ed25519,
}
