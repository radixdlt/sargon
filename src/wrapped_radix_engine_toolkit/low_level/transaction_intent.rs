use crate::prelude::*;
use radix_engine_toolkit::functions::intent::hash as ret_hash_intent;
use transaction::model::{
    InstructionsV1 as ScryptoInstructions, IntentHash as ScryptoIntentHash,
    IntentV1 as ScryptoIntent, MessageV1 as ScryptoMessage,
};

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct TransactionIntent {
    pub header: TransactionHeader,
    pub manifest: TransactionManifest,
    pub message: Message,
}

impl TransactionIntent {
    pub fn network_id(&self) -> NetworkID {
        self.header.network_id
    }

    pub fn new(
        header: TransactionHeader,
        manifest: TransactionManifest,
        message: Message,
    ) -> Self {
        Self {
            header,
            manifest,
            message,
        }
    }

    pub fn intent_hash(&self) -> Result<IntentHash> {
        ret_hash_intent(&self.clone().into())
            .map_err(|e| {
                error!("Failed to hash intent using RET, error: {:?}", e);
                CommonError::FailedToHashIntent
            })
            .map(|hash| {
                IntentHash::from_scrypto(
                    ScryptoIntentHash(hash.hash),
                    self.header.network_id,
                )
            })
    }

    pub fn compile(&self) -> Result<BagOfBytes> {
        todo!()
    }
}

impl From<TransactionIntent> for ScryptoIntent {
    fn from(value: TransactionIntent) -> Self {
        Self {
            header: value.header.into(),
            instructions: ScryptoInstructions(
                value.manifest.instructions().clone(),
            ),
            blobs: value.manifest.blobs().clone().into(),
            message: value.message.into(),
        }
    }
}
impl TryFrom<ScryptoIntent> for TransactionIntent {
    type Error = crate::CommonError;

    fn try_from(value: ScryptoIntent) -> Result<Self, Self::Error> {
        let message: Message = value.message.try_into()?;
        let header: TransactionHeader = value.header.try_into()?;
        let network_id = header.network_id;
        let instructions =
            Instructions::from_scrypto(value.instructions, network_id);
        let blobs: Blobs = value.blobs.into();
        let manifest = TransactionManifest::with_instructions_and_blobs(
            instructions,
            blobs,
        );

        Ok(Self {
            header,
            manifest,
            message,
        })
    }
}

impl HasSampleValues for TransactionIntent {
    fn sample() -> Self {
        Self::new(
            TransactionHeader::sample(),
            TransactionManifest::sample(),
            Message::sample(),
        )
    }

    // The Intent of:
    // https://github.com/radixdlt/radixdlt-scrypto/blob/ff21f24952318387803ae720105eec079afe33f3/transaction/src/model/hash/encoder.rs#L115
    fn sample_other() -> Self {
        Self::new(
            TransactionHeader::sample_other(),
            TransactionManifest::empty(NetworkID::Simulator),
            Message::None,
        )
    }
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
        let hash = SUT::sample().intent_hash().unwrap();
        assert_eq!(hash.to_string(), "txid_rdx12nnrygyt3p5v5pft5e3vu93v38qp5k7fh9v59kd6vtu8506880nq5vsxx6")
    }

    #[test]
    fn network_id() {
        assert_eq!(SUT::sample().network_id(), NetworkID::Mainnet);
        assert_eq!(SUT::sample_other().network_id(), NetworkID::Simulator);
    }

    #[test]
    fn to_from_scrypto() {
        let roundtrip = |s: SUT| {
            TryInto::<SUT>::try_into(Into::<ScryptoIntent>::into(s)).unwrap()
        };
        roundtrip(SUT::sample());
        roundtrip(SUT::sample_other());
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn compile() {
        _ = SUT::sample().compile()
    }
}
