use crate::prelude::*;
use delegate::delegate;
use std::hash::Hasher;

#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
#[debug("header:\n{:?}\n\nmessage:\n{:?}\n\nmanifest:\n{}\n\n", self.header, self.message, self.manifest.manifest_string())]
pub struct Subintent {
    pub header: IntentHeaderV2,
    pub manifest: SubintentManifest,
    pub message: MessageV2,
}

impl Subintent {
    pub fn new(
        header: IntentHeaderV2,
        manifest: SubintentManifest,
        message: MessageV2,
    ) -> Result<Self> {
        _ = compile_intent_with(&header, &manifest, &message)?;
        Ok(Self {
            header,
            manifest,
            message,
        })
    }

    pub fn hash(&self) -> SubintentHash {
        let hash = ret_hash_subintent(&ScryptoSubintent::from(self.clone()))
            .expect("Should never fail to hash an intent. Sargon should only produce valid Intents");

        SubintentHash::from_scrypto(
            ScryptoSubintentHash(hash.hash),
            self.network_id(),
        )
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

fn into_scrypto(
    header: &IntentHeaderV2,
    manifest: &SubintentManifest,
    message: &MessageV2,
) -> ScryptoSubintent {
    ScryptoSubintent {
        intent_core: ScryptoIntentCoreV2 {
            header: (*header).into(),
            blobs: manifest.blobs.clone().into(),
            message: message.clone().into(),
            children: manifest.children.clone().into(),
            instructions: ScryptoInstructionsV2(
                manifest.instructions().clone(),
            ),
        },
    }
}

fn compile_intent_with(
    header: &IntentHeaderV2,
    manifest: &SubintentManifest,
    message: &MessageV2,
) -> Result<BagOfBytes> {
    compile_intent(into_scrypto(header, manifest, message))
}

fn compile_intent(scrypto_intent: ScryptoSubintent) -> Result<BagOfBytes> {
    RET_subintent_to_payload_bytes(&scrypto_intent)
        .map_err(|e| CommonError::InvalidIntentFailedToEncode {
            underlying: format!("{:?}", e),
        })
        .map(BagOfBytes::from)
}

impl From<Subintent> for ScryptoSubintent {
    fn from(value: Subintent) -> Self {
        into_scrypto(&value.header, &value.manifest, &value.message)
    }
}

impl TryFrom<ScryptoSubintent> for Subintent {
    type Error = CommonError;

    fn try_from(
        value: ScryptoSubintent,
    ) -> std::result::Result<Self, Self::Error> {
        let network_id =
            NetworkID::try_from(value.intent_core.header.network_id)?;

        let manifest =
            TryFrom::<(ScryptoSubintentManifestV2, NetworkID)>::try_from((
                ScryptoSubintentManifestV2::from_intent_core(
                    &value.intent_core,
                ),
                network_id,
            ))?;
        let header = TryFrom::<ScryptoIntentHeaderV2>::try_from(
            value.intent_core.header.clone(),
        )?;
        let message =
            TryFrom::<ScryptoMessageV2>::try_from(value.intent_core.message)?;

        Self::new(header, manifest, message)
    }
}

impl std::hash::Hash for Subintent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.hash().hash.as_ref())
    }
}

impl HasSampleValues for Subintent {
    fn sample() -> Self {
        Self::new(
            IntentHeaderV2::sample(),
            SubintentManifest::sample(),
            MessageV2::sample(),
        )
        .unwrap()
    }

    fn sample_other() -> Self {
        Self::new(
            IntentHeaderV2::sample_other(),
            SubintentManifest::empty(NetworkID::Simulator),
            MessageV2::None,
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::process::Child;

    use super::*;
    use radix_transactions::manifest::CallMethod;
    use sbor::ValueKind as ScryptoValueKind;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Subintent;

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
    fn compile() {
        assert_eq!(SUT::sample().compile().to_string(), "4d220b012105210607010a872c0100000000000a912c01000000000022010105008306670000000022010105e8860667000000000a15cd5b070000000020200022010121020c0a746578742f706c61696e2200010c0c48656c6c6f205261646978212020002022054103800051c9a978fb5bfa066a3e5658251ee3304fb9bf58c35b61f8c10e0e7b91840c086c6f636b5f6665652101850000fda0c4277708000000000000000000000000000000004103800051c9a978fb5bfa066a3e5658251ee3304fb9bf58c35b61f8c10e0e7b91840c087769746864726177210280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a48000000000000000000000000000000000280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a480000000000000000000000000000004103800051ac224ee242c339b5ea5f1ae567f0520a6ffa24b52a10b8e6cd96a8347f0c147472795f6465706f7369745f6f725f61626f72742102810000000022000060012100");
    }

    #[test]
    fn subintent_hash() {
        let hash = SUT::sample().hash();
        assert_eq!(hash.to_string(), "subtxid_rdx1xput628m2l7jjweefd70gnq3t3a5x2gjeljduwm7vwly94s8ullql92sa0")
    }

    #[test]
    fn network_id() {
        assert_eq!(SUT::sample().network_id(), NetworkID::Mainnet);
    }

    #[test]
    fn manifest_string() {
        assert_eq!(
            SUT::sample().manifest_string(),
            "CallMethod { method: \"text/plain\", data: \"Hello Radix!  \" }"
        );
    }

    #[test]
    fn to_from_scrypto() {
        let roundtrip = |s: SUT| SUT::try_from(ScryptoSubintent::from(s)).unwrap();
        roundtrip(SUT::sample());
        roundtrip(SUT::sample_other());
    }
}
