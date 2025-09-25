use radix_transactions::model::TransactionPayload as _;

use crate::prelude::*;

#[derive(
    Serialize,
    Deserialize, /* Deserialize so we can test roundtrip of JSON vectors */
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub struct TransactionPreviewRequestV2 {
    /**
     * A hex-encoded, compiled RawPreviewTransaction.
     */
    pub preview_transaction: PreviewTransactionV2,

    pub flags: TransactionPreviewRequestFlags,

    /** A set of flags to configure the response of the transaction preview. */
    pub opt_ins: TransactionPreviewRequestOptInsV2,
}

#[derive(
    Serialize,
    Deserialize, /* Deserialize so we can test roundtrip of JSON vectors */
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub struct PreviewTransactionV2 {
    #[serde(rename = "type")]
    pub transaction_type: PreviewTransactionTypeV2,
    pub preview_transaction_hex: String,
}

#[derive(
    Serialize,
    Deserialize, /* Deserialize so we can test roundtrip of JSON vectors */
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub enum PreviewTransactionTypeV2 {
    Compiled,
}

impl TransactionPreviewRequestV2 {
    pub fn new_with_encoded_preview(encoded_preview: String) -> Self {
        Self {
            preview_transaction: PreviewTransactionV2 {
                transaction_type: PreviewTransactionTypeV2::Compiled,
                preview_transaction_hex: encoded_preview,
            },
            flags: TransactionPreviewRequestFlags::default(),
            opt_ins: TransactionPreviewRequestOptInsV2::default(),
        }
    }
}

impl TransactionPreviewRequestV2 {
    pub fn new_transaction_analysis(
        manifest: TransactionManifestV2,
        start_epoch_inclusive: Epoch,
        signer_public_keys: impl IntoIterator<Item = PublicKey>,
        notary_public_key: PublicKey,
        nonce: Nonce,
    ) -> Result<Self> {
        let signer_public_keys = signer_public_keys
            .into_iter()
            .map(ScryptoPublicKey::from)
            .collect_vec();

        let header = ScryptoTransactionHeaderV2 {
            notary_public_key: notary_public_key.into(),
            notary_is_signatory: signer_public_keys.is_empty(),
            tip_basis_points: 0,
        };
        let intent_header = ScryptoIntentHeaderV2 {
            network_id: manifest.network_id().discriminant(),
            start_epoch_inclusive: start_epoch_inclusive.into(),
            end_epoch_exclusive: Epoch::window_end_from_start(
                start_epoch_inclusive,
            )
            .into(),
            min_proposer_timestamp_inclusive: None,
            max_proposer_timestamp_exclusive: None,
            intent_discriminator: nonce.0 as u64,
        };

        let preview_transaction = ScryptoTransactionV2Builder::new()
            .manifest(manifest.scrypto_manifest())
            .transaction_header(header)
            .intent_header(intent_header)
            .build_preview_transaction(signer_public_keys);

        let encoded_preview_transaction = preview_transaction
            .to_raw()
            .map_err(|err| CommonError::FailedToEncodeTransactionPreviewV2 {
                underlying: format!("{:?}", err),
            })?
            .to_hex();

        Ok(Self::new_with_encoded_preview(encoded_preview_transaction))
    }
}
