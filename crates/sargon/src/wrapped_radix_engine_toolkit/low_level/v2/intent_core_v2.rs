use crate::prelude::*;

/// Represents the core of an intent in version 2, including the header,
/// manifest, and message. Used in both Subintent and TransactionIntent.
#[derive(Clone, PartialEq, Eq, derive_more::Debug, uniffi::Record)]
#[debug("header:\n{:?}\n\nmessage:\n{:?}\n\nmanifest:\n{}\n\n", self.header, self.message, self.manifest.manifest_string())]
pub struct IntentCoreV2 {
    pub header: IntentHeaderV2,
    manifest: TransactionManifestV2,
    pub message: MessageV2,
}

impl IntentCoreV2 {
    pub fn new(
        header: IntentHeaderV2,
        manifest: TransactionManifestV2,
        message: MessageV2,
    ) -> Result<Self> {
        // Verify that this IntentCoreV2 has acceptable depth and is compatible
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

    pub fn subintent_hash(&self) -> SubintentHash {
        let hash = ret_hash_intent_core_v2(&ScryptoIntentCoreV2::from(self.clone()))
            .expect("Should never fail to hash an intent. Sargon should only produce valid Intents");

        SubintentHash::from_scrypto(
            ScryptoSubintentHash(hash),
            self.header.network_id,
        )
    }

    pub fn compile(&self) -> BagOfBytes {
        compile_intent(ScryptoIntentCoreV2::from(self.clone()))
            .expect("Should always be able to compile an Intent")
    }
}

impl From<IntentCoreV2> for ScryptoIntentCoreV2 {
    fn from(value: IntentCoreV2) -> Self {
        into_scrypto(&value.header, &value.manifest, &value.message)
    }
}

fn into_scrypto(
    header: &IntentHeaderV2,
    manifest: &TransactionManifestV2,
    message: &MessageV2,
) -> ScryptoIntentCoreV2 {
    ScryptoIntentCoreV2 {
        header: (*header).into(),
        blobs: manifest.blobs().clone().into(),
        message: message.clone().into(),
        children: manifest.children().clone().into(),
        instructions: ScryptoInstructionsV2(
            manifest.instructions().clone().into(),
        ),
    }
}

fn compile_intent_with(
    header: &IntentHeaderV2,
    manifest: &TransactionManifestV2,
    message: &MessageV2,
) -> Result<BagOfBytes> {
    compile_intent(into_scrypto(header, manifest, message))
}

fn compile_intent(scrypto_intent: ScryptoIntentCoreV2) -> Result<BagOfBytes> {
    RET_intent_to_payload_bytes_v2(&scrypto_intent)
        .map_err(|e| CommonError::InvalidIntentFailedToEncode {
            underlying: format!("{:?}", e),
        })
        .map(BagOfBytes::from)
}

impl TryFrom<ScryptoIntentCoreV2> for IntentCoreV2 {
    type Error = crate::CommonError;

    fn try_from(value: ScryptoIntentCoreV2) -> Result<Self, Self::Error> {
        let message: MessageV2 = value.message.try_into()?;
        let header: IntentHeaderV2 = value.header.try_into()?;
        let network_id = header.network_id;
        let instructions = InstructionsV2::try_from((
            value.instructions.0.as_ref(),
            network_id,
        ))?;
        let blobs: Blobs = value.blobs.into();
        let children: ChildIntents =
            (value.children.children, network_id).into();
        let manifest =
            TransactionManifestV2::with_instructions_and_blobs_and_children(
                instructions,
                blobs,
                children,
            );

        Self::new(header, manifest, message)
    }
}

impl HasSampleValues for IntentCoreV2 {
    fn sample() -> Self {
        Self::new(
            IntentHeaderV2::sample(),
            TransactionManifestV2::sample(),
            MessageV2::sample(),
        )
        .unwrap()
    }

    fn sample_other() -> Self {
        Self::new(
            IntentHeaderV2::sample_other(),
            TransactionManifestV2::empty(NetworkID::Simulator),
            MessageV2::None,
        )
        .unwrap()
    }
}

#[cfg(test)]
impl IntentCoreV2 {
    /// Utility function which uses `IntentCoreV2::new(<IntentHeaderV2>, <TransactionManifestV2>, <MessageV2>)`
    /// and SHOULD return `Err` if `depth > IntentCoreV2::MAX_SBOR_DEPTH`, which
    /// we can assert in unit tests.
    pub(crate) fn test_with_sbor_depth(
        depth: usize,
        network_id: NetworkID,
    ) -> Result<Self> {
        InstructionsV2::test_with_sbor_depth(depth, network_id)
            .and_then(|instructions| {
                TransactionManifestV2::new(
                    instructions.instructions_string(),
                    network_id,
                    Blobs::default(),
                    ChildIntents::empty(),
                )
            })
            .and_then(|manifest| {
                Self::new(
                    IntentHeaderV2::sample(),
                    manifest,
                    MessageV2::sample(),
                )
            })
    }

    pub(crate) const MAX_SBOR_DEPTH: usize = InstructionsV2::MAX_SBOR_DEPTH;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = IntentCoreV2;

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
    fn subintent_hash() {
        let hash = SUT::sample().subintent_hash();
        assert_eq!(hash.to_string(), "subtxid_rdx1467nwzqa7g8vllzhh4jvtgh9rkv4k3qg6qnznywsytkrnztnn50q5kp3uu")
    }

    #[test]
    fn network_id() {
        assert_eq!(SUT::sample().network_id(), NetworkID::Mainnet);
        assert_eq!(SUT::sample_other().network_id(), NetworkID::Simulator);
    }

    #[test]
    fn to_from_scrypto() {
        let roundtrip =
            |s: SUT| SUT::try_from(ScryptoIntentCoreV2::from(s)).unwrap();
        roundtrip(SUT::sample());
        roundtrip(SUT::sample_other());
    }

    #[test]
    fn compile() {
        assert_eq!(SUT::sample().compile().to_string(), "4d2105210607010a872c0100000000000a912c01000000000022010105008306670000000022010105e8860667000000000a15cd5b070000000020200022010121020c0a746578742f706c61696e2200010c0c48656c6c6f20526164697821202001072048f1bd08444b5e713db9e14caac2faae71836786ac94d645b00679728202a93520220212001300");
    }

    #[test]
    fn intent_with_max_sbor_depth_is_ok() {
        let sut =
            SUT::test_with_sbor_depth(SUT::MAX_SBOR_DEPTH, NetworkID::Stokenet)
                .unwrap();
        println!("{}", &sut.manifest);
        assert_eq!(sut.subintent_hash().to_string(), "subtxid_rdx1pcn6caff4sq8ayclqp7nkwvqux77895j9pkx59p0rd5r059ntt2s22zvvk")
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
}
