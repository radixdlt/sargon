use crate::prelude::*;
use transaction::model::{
    NotarizedTransactionV1 as ScryptoNotarizedTransaction,
    SignedIntentV1 as ScryptoSignedIntent,
};

use radix_engine_toolkit::functions::notarized_transaction::compile as RET_compile_notarized_tx;

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
}

impl From<NotarizedTransaction> for ScryptoNotarizedTransaction {
    fn from(value: NotarizedTransaction) -> Self {
        ScryptoNotarizedTransaction {
            signed_intent: value.signed_intent.into(),
            notary_signature: value.notary_signature.into(),
        }
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
    fn test_compile() {
        // FIXME: replace with cross checked values, these values have just
        // been recorded here to catch if the value ever changes
        assert_eq!(SUT::sample().compile().to_hex(), "4d22030221022104210707010a872c0100000000000a912c01000000000009092f2400220101200720ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf010108000020220441038000d1be9c042f627d98a01383987916d43cf439631ca1d8c8076d6754ab263d0c086c6f636b5f6665652101850000fda0c42777080000000000000000000000000000000041038000d1be9c042f627d98a01383987916d43cf439631ca1d8c8076d6754ab263d0c087769746864726177210280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a48000000000000000000000000000000000280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a4800000000000000000000000000000041038000d1127918c16af09af521951adcf3a20ab2cc87c0e72e85814764853ce5e70c147472795f6465706f7369745f6f725f61626f72742102810000000022000020200022010121020c0a746578742f706c61696e2200010c0c48656c6c6f205261646978212022002201012101200740839ac9c47db45950fc0cd453c5ebbbfa7ae5f7c20753abe2370b5b40fdee89e522c4d810d060e0c56211d036043fd32b9908e97bf114c1835ca02d74018fdd09")
    }

    #[test]
    fn test_compile_other() {
        // FIXME: replace with cross checked values, these values have just
        // been recorded here to catch if the value ever changes
        assert_eq!(SUT::sample_other().compile().to_hex(), "4d22030221022104210707f20a00000000000000000a0a00000000000000090a0000002200012007210279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f817980101080000202200202000220000202200220001210120074101ebfc1f10a3b6ed83531f16249477ab86b77ce85980ef330abafbbd758caa98c665f68b8536112b6d1519feddeea01fd8429124dd75121d4bd88c14a27b68a123")
    }
}
