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

impl Default for TransactionPreviewRequestFlags {
    fn default() -> Self {
        Self {
            use_free_credit: true,
            assume_all_signature_proofs: false,
            skip_epoch_check: false,
        }
    }
}

impl From<PublicKey> for GWPublicKey {
    fn from(value: PublicKey) -> Self {
        match value {
            PublicKey::Ed25519 { value } => Self::Ed25519(value),
            PublicKey::Secp256k1 { value } => Self::Secp256k1(value),
        }
    }
}
