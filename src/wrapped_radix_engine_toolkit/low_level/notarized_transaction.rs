use crate::prelude::*;
use transaction::model::{
    NotarizedTransactionV1 as ScryptoNotarizedTransaction,
    SignedIntentV1 as ScryptoSignedIntent,
};

#[derive(Debug, Clone, Eq, PartialEq, uniffi::Record)]
pub struct NotarizedTransaction {
    pub signed_intent: SignedIntent,
    pub notary_signature: NotarySignature,
}

impl NotarizedTransaction {
    pub fn new(
        signed_intent: SignedIntent,
        notary_signature: NotarySignature,
    ) -> Self {
        Self {
            signed_intent,
            notary_signature,
        }
    }

    pub fn compile(&self) -> Result<BagOfBytes> {
        todo!()
    }
}

impl From<NotarizedTransaction> for ScryptoNotarizedTransaction {
    fn from(value: NotarizedTransaction) -> Self {
        ScryptoNotarizedTransaction {
            signed_intent: value.signed_intent.into(),
            notary_signature: value.notary_signature.into(),
        }
    }
}

impl TryFrom<ScryptoNotarizedTransaction> for NotarizedTransaction {
    type Error = crate::CommonError;

    fn try_from(
        value: ScryptoNotarizedTransaction,
    ) -> Result<Self, Self::Error> {
        let signed_intent: SignedIntent = value.signed_intent.try_into()?;
        Ok(Self {
            signed_intent,
            notary_signature: value.notary_signature.into(),
        })
    }
}

impl HasSampleValues for NotarizedTransaction {
    fn sample() -> Self {
        let private_key = Ed25519PrivateKey::sample_alice();
        let intent = TransactionIntent::sample();

        let signed_intent =
            SignedIntent::new(intent, IntentSignatures::default());

        let signed_intent_hash = signed_intent.hash().unwrap();

        Self::new(
            signed_intent,
            private_key.notarize_hash(&signed_intent_hash),
        )
    }

    // Identical to: https://github.com/radixdlt/radixdlt-scrypto/blob/ff21f24952318387803ae720105eec079afe33f3/transaction/src/model/hash/encoder.rs#L115
    // intent hash: `"60e5617d670e6c8a42ba5f3749f4ff1079f66221f282554ecdda9ad385ecb195"`
    // bech32 encoded   (mainnet): `"txid_rdx1vrjkzlt8pekg5s46tum5na8lzpulvc3p72p92nkdm2dd8p0vkx2syss63y"`
    // bech32 encoded (simulator): `"txid_sim1vrjkzlt8pekg5s46tum5na8lzpulvc3p72p92nkdm2dd8p0vkx2svr7ejr"`
    fn sample_other() -> Self {
        let private_key: Secp256k1PrivateKey =
            radix_engine::types::Secp256k1PrivateKey::from_u64(1)
                .unwrap()
                .into();

        let intent = TransactionIntent::sample_other();
        assert_eq!(intent.intent_hash().unwrap().to_string(), "txid_sim1vrjkzlt8pekg5s46tum5na8lzpulvc3p72p92nkdm2dd8p0vkx2svr7ejr");
        let signed_intent =
            SignedIntent::new(intent, IntentSignatures::new(Vec::new()));

        let signed_intent_hash = signed_intent.hash().unwrap();

        Self::new(
            signed_intent,
            private_key.notarize_hash(&signed_intent_hash),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NotarizedTransaction;

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
    fn to_from_scrypto() {
        let roundtrip = |s: SUT| {
            TryInto::<SUT>::try_into(Into::<ScryptoNotarizedTransaction>::into(
                s,
            ))
            .unwrap()
        };
        roundtrip(SUT::sample());
        roundtrip(SUT::sample_other());
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn todo_compile() {
        _ = SUT::sample().compile();
    }
}
