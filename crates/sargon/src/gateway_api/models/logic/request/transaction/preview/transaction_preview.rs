use crate::prelude::*;

impl TransactionPreviewRequest {
    pub fn new_transaction_analysis_v1(
        manifest: TransactionManifest,
        start_epoch_inclusive: Epoch,
        signer_public_keys: impl IntoIterator<Item = PublicKey>,
        notary_public_key: PublicKey,
        nonce: Nonce,
    ) -> Self {
        let signer_public_keys = signer_public_keys
            .into_iter()
            .map(GWPublicKey::from)
            .collect_vec();

        Self {
            manifest: manifest.manifest_string(),
            blobs_hex: Some(
                manifest
                    .blobs()
                    .blobs()
                    .into_iter()
                    .map(|b| b.to_hex())
                    .collect_vec(),
            ),
            start_epoch_inclusive: start_epoch_inclusive.into(),
            end_epoch_exclusive: Epoch::window_end_from_start(
                start_epoch_inclusive,
            )
            .into(),
            notary_public_key: Some(GWPublicKey::from(notary_public_key)),
            notary_is_signatory: signer_public_keys.is_empty(),
            tip_percentage: 0,
            nonce: nonce.into(),
            signer_public_keys: signer_public_keys,
            flags: TransactionPreviewRequestFlags::default(),
            opt_ins: TransactionPreviewRequestOptIns::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionPreviewRequest;

    #[test]
    fn test_new() {
        let do_test = |intent: TransactionIntent| {
            let header = intent.header;
            let keys = vec![PublicKey::sample(), PublicKey::sample_other()];
            let flags = TransactionPreviewRequestFlags::default();
            let sut = SUT::new_transaction_analysis_v1(
                intent.clone().manifest,
                intent.header.start_epoch_inclusive,
                keys.clone(),
                intent.header.notary_public_key,
                intent.header.nonce,
            );
            assert_eq!(sut.flags, flags);
            assert_eq!(
                sut.signer_public_keys,
                keys.into_iter().map(GWPublicKey::from).collect_vec()
            );
            assert_eq!(sut.manifest, intent.manifest_string());
            assert_eq!(
                Epoch::from(sut.start_epoch_inclusive),
                header.start_epoch_inclusive
            );
            assert_eq!(
                Epoch::from(sut.end_epoch_exclusive),
                header.end_epoch_exclusive
            );
            assert_eq!(
                sut.blobs_hex.unwrap(),
                intent
                    .clone()
                    .blobs()
                    .blobs()
                    .into_iter()
                    .map(|b| b.to_string())
                    .collect_vec()
            );
            assert_eq!(sut.notary_is_signatory, header.notary_is_signatory);
            assert_eq!(
                sut.notary_public_key.unwrap(),
                GWPublicKey::from(header.notary_public_key)
            );
            assert_eq!(sut.tip_percentage, header.tip_percentage);
            assert_eq!(Nonce::from(sut.nonce), header.nonce);
        };
        do_test(TransactionIntent::sample());
        do_test(TransactionIntent::sample_other());
    }

    #[test]
    fn request_json_test() {
        let (sut, json) = fixture_and_json::<SUT>(include_str!(concat!(
            env!("FIXTURES_MODELS_GW"),
            "transaction/request_preview.json"
        )))
        .unwrap();
        assert_json_value_eq_after_roundtrip(&sut, json)
    }
}
