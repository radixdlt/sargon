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
    #[serde(rename = "nonce")]
    pub intent_discriminator: u32,

    /** A list of public keys to be used as transaction signers */
    pub signer_public_keys: Vec<GWPublicKey>,

    pub flags: TransactionPreviewRequestFlags,

    /** A set of flags to configure the response of the transaction preview. */
    pub opt_ins: TransactionPreviewRequestOptIns,
}

impl TransactionPreviewRequest {
    pub fn new_transaction_analysis(
        manifest: TransactionManifest,
        start_epoch_inclusive: Epoch,
        signer_public_keys: impl IntoIterator<Item = PublicKey>,
        notary_public_key: Option<PublicKey>,
        intent_discriminator: IntentDisciminator32,
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
            notary_public_key: notary_public_key.map(GWPublicKey::from),
            notary_is_signatory: signer_public_keys.is_empty(),
            tip_percentage: 0,
            intent_discriminator: intent_discriminator.into(),
            signer_public_keys,
            flags: TransactionPreviewRequestFlags::default(),
            opt_ins: TransactionPreviewRequestOptIns::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use prelude::fixture_gw_model;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionPreviewRequest;

    #[test]
    fn test_new() {
        let do_test = |intent: TransactionIntent| {
            let header = intent.header;
            let keys = vec![PublicKey::sample(), PublicKey::sample_other()];
            let flags = TransactionPreviewRequestFlags::default();
            let sut = SUT::new_transaction_analysis(
                intent.clone().manifest,
                intent.header.start_epoch_inclusive,
                keys.clone(),
                Some(intent.header.notary_public_key),
                intent.header.intent_discriminator,
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
            assert!(!sut.notary_is_signatory);
            assert_eq!(
                sut.notary_public_key.unwrap(),
                GWPublicKey::from(header.notary_public_key)
            );
            assert_eq!(sut.tip_percentage, header.tip_percentage);
            assert_eq!(IntentDisciminator32::from(sut.intent_discriminator), header.intent_discriminator);
        };
        do_test(TransactionIntent::sample());
        do_test(TransactionIntent::sample_other());
    }

    #[test]
    fn request_json_test() {
        let (sut, json) = fixture_and_json::<SUT>(fixture_gw_model!(
            "transaction/request_preview"
        ))
        .unwrap();
        assert_json_value_eq_after_roundtrip(&sut, json)
    }
}
