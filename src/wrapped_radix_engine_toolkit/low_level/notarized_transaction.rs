use crate::prelude::*;
use transaction::model::{
    NotarizedTransactionV1 as ScryptoNotarizedTransaction,
    SignedIntentV1 as ScryptoSignedIntent,
};

use radix_engine_toolkit::functions::notarized_transaction::{
    compile as RET_compile_notarized_tx, decompile as RET_decompile_notarize_tx,
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

    pub fn compile(&self) -> BagOfBytes {
        let scrypto: ScryptoNotarizedTransaction = self.clone().into();
        RET_compile_notarized_tx(&scrypto)
            .expect("Should always be able to compile a notarized tx")
            .into()
    }

    pub fn decompile(compiled_notarized_tx: &BagOfBytes) -> Result<Self> {
        RET_decompile_notarize_tx(compiled_notarized_tx.bytes())
        .map_err(|e| {
            error!("Failed to decompile bytes into Notarized Transaction, error: {:?}", e);
            CommonError::FailedToDecompileBytesIntoNotarizedTransaction
        })
        .and_then(TryInto::<Self>::try_into)
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

        let signed_intent = SignedIntent::new_validating_signatures(
            intent,
            IntentSignatures::default(),
        )
        .unwrap();

        let signed_intent_hash = signed_intent.hash();

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
        assert_eq!(intent.intent_hash().to_string(), "txid_sim1vrjkzlt8pekg5s46tum5na8lzpulvc3p72p92nkdm2dd8p0vkx2svr7ejr");
        let signed_intent = SignedIntent::new_validating_signatures(
            intent,
            IntentSignatures::new(Vec::new()),
        )
        .unwrap();

        let signed_intent_hash = signed_intent.hash();

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
    fn test_compile_decompile_roundtrip() {
        let roundtrip = |s: SUT| {
            assert_eq!(SUT::decompile(&s.clone().compile()).unwrap(), s)
        };
        roundtrip(SUT::sample());
        roundtrip(SUT::sample_other())
    }
}
