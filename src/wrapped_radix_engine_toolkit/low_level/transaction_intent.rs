use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, derive_more::Debug, uniffi::Record)]
#[debug("header:\n{:?}\n\nmessage:\n{:?}\n\nmanifest:\n{}\n\n", self.header, self.message, self.manifest.instructions_string())]
pub struct TransactionIntent {
    pub header: TransactionHeader,
    manifest: TransactionManifest,
    pub message: Message,
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

    pub fn intent_hash(&self) -> IntentHash {
        let hash = ret_hash_intent(&ScryptoIntent::from(self.clone()))
          .expect("Should never fail to hash an intent. Sargon should only produce valid Intents");

        IntentHash::from_scrypto(
            ScryptoIntentHash(hash.hash),
            self.header.network_id,
        )
    }

    pub fn compile(&self) -> BagOfBytes {
        compile_intent(ScryptoIntent::from(self.clone()))
            .expect("Should always be able to compile an Intent")
    }
}

impl From<TransactionIntent> for ScryptoIntent {
    fn from(value: TransactionIntent) -> Self {
        into_scrypto(&value.header, &value.manifest, &value.message)
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
    RET_intent_compile(&scrypto_intent)
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
    fn intent_hash() {
        let hash = SUT::sample().intent_hash();
        assert_eq!(hash.to_string(), "txid_rdx12nnrygyt3p5v5pft5e3vu93v38qp5k7fh9v59kd6vtu8506880nq5vsxx6")
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
        assert_eq!(SUT::sample().compile().to_string(), "4d220104210707010a872c0100000000000a912c01000000000009092f2400220101200720ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf010108000020220441038000d1be9c042f627d98a01383987916d43cf439631ca1d8c8076d6754ab263d0c086c6f636b5f6665652101850000fda0c42777080000000000000000000000000000000041038000d1be9c042f627d98a01383987916d43cf439631ca1d8c8076d6754ab263d0c087769746864726177210280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a48000000000000000000000000000000000280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a4800000000000000000000000000000041038000d1127918c16af09af521951adcf3a20ab2cc87c0e72e85814764853ce5e70c147472795f6465706f7369745f6f725f61626f72742102810000000022000020200022010121020c0a746578742f706c61696e2200010c0c48656c6c6f20526164697821");
    }

    #[test]
    fn intent_with_max_sbor_depth_is_ok() {
        assert!(SUT::test_with_sbor_depth(
            SUT::MAX_SBOR_DEPTH,
            NetworkID::Stokenet
        )
        .is_ok());
    }

    #[test]
    fn intent_with_sbor_depth_greater_than_max_is_err() {
        assert_eq!(
            SUT::test_with_sbor_depth(
                SUT::MAX_SBOR_DEPTH + 1,
                NetworkID::Stokenet
            ),
            Err(CommonError::InvalidTransactionMaxSBORDepthExceeded(20))
        );
    }
}
