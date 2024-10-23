use crate::prelude::*;

/// Represents the core of an intent in V2, including the header, manifest, and message.
#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
#[debug("header:\n{:?}\n\nmessage:\n{:?}\n\nmanifest:\n{}\n\n", self.header, self.message, self.manifest.manifest_string())]
pub struct IntentCoreV2 {
    pub header: IntentHeaderV2,
    pub manifest: TransactionManifestV2,
    pub message: MessageV2,
}

impl IntentCoreV2 {
    pub fn new(
        header: IntentHeaderV2,
        manifest: TransactionManifestV2,
        message: MessageV2,
    ) -> Self {
        Self {
            header,
            manifest,
            message,
        }
    }

    pub fn network_id(&self) -> NetworkID {
        self.header.network_id
    }

    pub fn manifest_string(&self) -> String {
        self.manifest.manifest_string()
    }

    pub fn blobs(&self) -> &Blobs {
        &self.manifest.blobs
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
        blobs: manifest.blobs.clone().into(),
        message: message.clone().into(),
        children: manifest.children.clone().into(),
        instructions: ScryptoInstructionsV2(
            manifest.instructions().clone().into(),
        ),
    }
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

        Ok(Self::new(header, manifest, message))
    }
}

impl HasSampleValues for IntentCoreV2 {
    fn sample() -> Self {
        Self::new(
            IntentHeaderV2::sample(),
            TransactionManifestV2::sample(),
            MessageV2::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            IntentHeaderV2::sample_other(),
            TransactionManifestV2::empty(NetworkID::Simulator),
            MessageV2::None,
        )
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
                Ok(Self::new(
                    IntentHeaderV2::sample(),
                    manifest,
                    MessageV2::sample(),
                ))
            })
    }

    pub(crate) const MAX_SBOR_DEPTH: usize = InstructionsV2::MAX_SBOR_DEPTH;
}

#[cfg(test)]
mod tests {
    use super::*;
    use radix_transactions::manifest::CallMethod;
    use sbor::ValueKind as ScryptoValueKind;

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
    fn manifest_string() {
        let sut = SUT::sample();
        let manifest_string = SUT::sample().manifest_string();
        assert_eq!(manifest_string, sut.manifest.manifest_string())
    }

    #[test]
    fn blobs() {
        let sut = SUT::sample();
        assert_eq!(sut.blobs().clone(), Blobs::default());
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
    fn intent_with_max_sbor_depth_is_ok() {
        let sut =
            SUT::test_with_sbor_depth(SUT::MAX_SBOR_DEPTH, NetworkID::Stokenet);
        assert!(sut.is_ok());
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
