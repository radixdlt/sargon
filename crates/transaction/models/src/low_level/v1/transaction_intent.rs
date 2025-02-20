use crate::prelude::*;
use std::hash::Hasher;

#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
#[debug("header:\n{:?}\n\nmessage:\n{:?}\n\nmanifest:\n{}\n\n", self.header, self.message, self.manifest.manifest_string())]
pub struct TransactionIntent {
    pub header: TransactionHeader,
    pub manifest: TransactionManifest,
    pub message: Message,
}

impl From<SignedIntent> for TransactionIntent {
    fn from(val: SignedIntent) -> Self {
        val.intent
    }
}

impl TransactionIntent {
    pub fn new(
        header: TransactionHeader,
        manifest: TransactionManifest,
        message: Message,
    ) -> Result<Self> {
        // Verify that this TransactionIntent has acceptable depth and is compatible
        _ = compile_intent_with(&header, &manifest, &message)?;

        Ok(Self {
            header,
            manifest,
            message,
        })
    }

    pub fn network_id(&self) -> NetworkID {
        self.header.network_id
    }

    pub fn manifest_string(&self) -> String {
        self.manifest.manifest_string()
    }

    pub fn blobs(&self) -> &Blobs {
        self.manifest.blobs()
    }

    pub fn transaction_intent_hash(&self) -> TransactionIntentHash {
        let hash = ret_hash_intent(&ScryptoIntent::from(self.clone()))
          .expect("Should never fail to hash an intent. Sargon should only produce valid Intents");

        TransactionIntentHash::from_scrypto(
            ScryptoTransactionIntentHash(hash.hash),
            self.header.network_id,
        )
    }
}

impl From<TransactionIntent> for ScryptoIntent {
    fn from(value: TransactionIntent) -> Self {
        into_scrypto(&value.header, &value.manifest, &value.message)
    }
}

impl std::hash::Hash for TransactionIntent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.transaction_intent_hash().hash.as_ref())
    }
}

fn into_scrypto(
    header: &TransactionHeader,
    manifest: &TransactionManifest,
    message: &Message,
) -> ScryptoIntent {
    ScryptoIntent {
        header: (*header).into(),
        instructions: ScryptoInstructions(manifest.instructions().clone()),
        blobs: manifest.blobs().clone().into(),
        message: message.clone().into(),
    }
}

fn compile_intent_with(
    header: &TransactionHeader,
    manifest: &TransactionManifest,
    message: &Message,
) -> Result<BagOfBytes> {
    compile_intent(into_scrypto(header, manifest, message))
}

fn compile_intent(scrypto_intent: ScryptoIntent) -> Result<BagOfBytes> {
    RET_intent_to_payload_bytes(&scrypto_intent)
        .map_err(|e| CommonError::InvalidIntentFailedToEncode {
            underlying: format!("{:?}", e),
        })
        .map(BagOfBytes::from)
}

impl TryFrom<ScryptoIntent> for TransactionIntent {
    type Error = crate::CommonError;

    fn try_from(value: ScryptoIntent) -> Result<Self, Self::Error> {
        let message: Message = value.message.try_into()?;
        let header: TransactionHeader = value.header.try_into()?;
        let network_id = header.network_id;
        let instructions = Instructions::try_from((
            value.instructions.0.as_ref(),
            network_id,
        ))?;
        let blobs: Blobs = value.blobs.into();
        let manifest = TransactionManifest::with_instructions_and_blobs(
            instructions,
            blobs,
        );

        Self::new(header, manifest, message)
    }
}

impl HasSampleValues for TransactionIntent {
    fn sample() -> Self {
        Self::new(
            TransactionHeader::sample(),
            TransactionManifest::sample(),
            Message::sample(),
        )
        .unwrap()
    }

    // The Intent of:
    // https://github.com/radixdlt/radixdlt-scrypto/blob/ff21f24952318387803ae720105eec079afe33f3/transaction/src/model/hash/encoder.rs#L115
    fn sample_other() -> Self {
        Self::new(
            TransactionHeader::sample_other(),
            TransactionManifest::empty(NetworkID::Simulator),
            Message::None,
        )
        .unwrap()
    }
}

#[cfg(test)]
impl TransactionIntent {
    /// Utility function which uses `TransactionIntent::new(<TransactionHeader>, <TransactionManifest>, <Message>)`
    /// and SHOULD return `Err` if `depth > TransactionIntent::MAX_SBOR_DEPTH`, which
    /// we can assert in unit tests.
    pub(crate) fn test_with_sbor_depth(
        depth: usize,
        network_id: NetworkID,
    ) -> Result<Self> {
        Instructions::test_with_sbor_depth(depth, network_id)
            .and_then(|instructions| {
                TransactionManifest::new(
                    instructions.instructions_string(),
                    network_id,
                    Blobs::default(),
                )
            })
            .and_then(|manifest| {
                Self::new(
                    TransactionHeader::sample(),
                    manifest,
                    Message::sample(),
                )
            })
    }

    pub(crate) const MAX_SBOR_DEPTH: usize = Instructions::MAX_SBOR_DEPTH;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionIntent;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn transaction_intent_hash() {
        let hash = SUT::sample().transaction_intent_hash();
        assert_eq!(hash.to_string(), "txid_rdx198k527d5wt4ms5tvrdcu8089v4hptp7ztv388k539uzzvmw25ltsj7u4zz")
    }

    #[test]
    fn network_id() {
        assert_eq!(SUT::sample().network_id(), NetworkID::Mainnet);
        assert_eq!(SUT::sample_other().network_id(), NetworkID::Simulator);
    }

    #[test]
    fn to_from_scrypto() {
        let roundtrip = |s: SUT| SUT::try_from(ScryptoIntent::from(s)).unwrap();
        roundtrip(SUT::sample());
        roundtrip(SUT::sample_other());
    }

    #[test]
    fn compile() {
        assert_eq!(SUT::sample().compile().to_string(), "4d220104210707010a872c0100000000000a912c01000000000009092f2400220101200720ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf01010800002022044103800051c9a978fb5bfa066a3e5658251ee3304fb9bf58c35b61f8c10e0e7b91840c086c6f636b5f6665652101850000fda0c4277708000000000000000000000000000000004103800051c9a978fb5bfa066a3e5658251ee3304fb9bf58c35b61f8c10e0e7b91840c087769746864726177210280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a48000000000000000000000000000000000280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a480000000000000000000000000000004103800051ac224ee242c339b5ea5f1ae567f0520a6ffa24b52a10b8e6cd96a8347f0c147472795f6465706f7369745f6f725f61626f72742102810000000022000020200022010121020c0a746578742f706c61696e2200010c0c48656c6c6f20526164697821");
    }

    #[test]
    fn intent_with_max_sbor_depth_is_ok() {
        let sut =
            SUT::test_with_sbor_depth(SUT::MAX_SBOR_DEPTH, NetworkID::Stokenet)
                .unwrap();
        assert_eq!(sut.transaction_intent_hash().to_string(), "txid_rdx1uwcfczupvvrrtxwxx6p5jugaxvu3j83tj5nz9pnrr44jyxccg2cqhuvzhy")
    }

    #[test]
    fn intent_with_sbor_depth_greater_than_max_is_err() {
        assert_eq!(
            SUT::test_with_sbor_depth(
                SUT::MAX_SBOR_DEPTH + 1,
                NetworkID::Stokenet
            ),
            Err(CommonError::InvalidTransactionMaxSBORDepthExceeded {
                max: 20_u16
            })
        );
    }

    #[test]
    fn other_reasons_for_invalid() {
        let res = compile_intent(invalid_signed_intent().intent);
        assert_eq!(
            res,
            Err(CommonError::InvalidIntentFailedToEncode { underlying: "MismatchingArrayElementValueKind { element_value_kind: 7, actual_value_kind: 8 }".to_owned() }) 
        );
    }
}
