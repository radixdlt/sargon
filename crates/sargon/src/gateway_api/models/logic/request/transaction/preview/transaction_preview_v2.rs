use crate::prelude::*;
use radix_transactions::model::TransactionPayload;

impl TransactionPreviewRequestV2 {
    pub fn new_transaction_analysis(
        manifest: ScryptoTransactionManifestV2,
        start_epoch_inclusive: Epoch,
        signer_public_keys: impl IntoIterator<Item = PublicKey>,
        notary_public_key: PublicKey,
        discriminator: IntentDiscriminator,
        network_id: NetworkID,
    ) -> Self {
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

            let preview = transaction_v2_builder.build_preview_transaction(signer_public_keys);
    
            Self {
                preview_transaction: PreviewTransaction {
                    transaction_type: PreviewTransactionType::Compiled,
                    preview_transaction_hex: preview.to_raw().unwrap().to_hex(),
                },
                flags: TransactionPreviewRequestFlags::default(),
                opt_ins: TransactionPreviewRequestOptInsV2 { core_api_receipt: true, radix_engine_toolkit_receipt: true, logs: false }
            }
    }
}