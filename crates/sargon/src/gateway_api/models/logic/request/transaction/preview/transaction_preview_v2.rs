use crate::prelude::*;
use radix_transactions::model::TransactionPayload;

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
        manifest: ScryptoTransactionManifestV2,
        start_epoch_inclusive: Epoch,
        signer_public_keys: impl IntoIterator<Item = PublicKey>,
        notary_public_key: PublicKey,
        discriminator: IntentDiscriminator,
        network_id: NetworkID,
    ) -> Result<Self> {
        let signer_public_keys = signer_public_keys
        .into_iter()
        .map(ScryptoPublicKey::from)
        .collect_vec();

            let mut transaction_v2_builder = ScryptoTransactionV2Builder::new();

            let header = ScryptoTransactionHeaderV2 {
                notary_public_key: notary_public_key.into(),
                notary_is_signatory: signer_public_keys.is_empty(),
                tip_basis_points: 0
            };
            let intent_header = ScryptoIntentHeaderV2 {
                network_id: network_id.discriminant(),
                start_epoch_inclusive: start_epoch_inclusive.into(),
                end_epoch_exclusive: Epoch::window_end_from_start(
                    start_epoch_inclusive,
                )
                .into(),
                min_proposer_timestamp_inclusive: None,
                max_proposer_timestamp_exclusive: None,
                intent_discriminator: discriminator.0,
            };
            
            transaction_v2_builder = transaction_v2_builder.manifest(manifest);
            transaction_v2_builder = transaction_v2_builder.transaction_header(header);
            transaction_v2_builder = transaction_v2_builder.intent_header(intent_header);

            let preview_transaction = transaction_v2_builder.build_preview_transaction(signer_public_keys);
            let encoded_preview_transaction = preview_transaction
            .to_raw()
            .map_err(|err| {
                CommonError::FailedToEncodeTransactionPreviewV2 { underlying: format!("{:?}", err) }
            })?
            .to_hex();
    
            Ok(Self::new_with_encoded_preview(encoded_preview_transaction))
    }
}