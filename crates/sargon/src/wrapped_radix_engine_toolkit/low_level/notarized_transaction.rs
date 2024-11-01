use crate::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NotarizedTransaction {
    pub signed_intent: SignedIntent,
    pub notary_signature: NotarySignature,
}

impl NotarizedTransaction {
    pub fn new(
        signed_intent: SignedIntent,
        notary_signature: NotarySignature,
    ) -> Result<Self> {
        // Verify that this NotarizedTransaction has acceptable depth and is compatible
        _ = compile_notarized_intent_with(&signed_intent, &notary_signature)?;

        Ok(Self {
            signed_intent,
            notary_signature,
        })
    }

    pub fn signed_intent(&self) -> &SignedIntent {
        &self.signed_intent
    }

    pub fn compile(&self) -> CompiledNotarizedIntent {
        compile_notarized_intent(self.clone().into())
        .expect("Should always be able to compile a NotarizedTransaction since we should have called 'compile' in its failing constructor.")
    }

    pub fn decompile(compiled_notarized_tx: &CompiledNotarizedIntent) -> Self {
        compiled_notarized_tx.decompile()
    }
}

impl From<NotarizedTransaction> for ScryptoNotarizedTransaction {
    fn from(value: NotarizedTransaction) -> Self {
        into_scrypto(&value.signed_intent, &value.notary_signature)
    }
}

impl TryFrom<ScryptoNotarizedTransaction> for NotarizedTransaction {
    type Error = crate::CommonError;

    fn try_from(
        value: ScryptoNotarizedTransaction,
    ) -> Result<Self, Self::Error> {
        let signed_intent: SignedIntent = value.signed_intent.try_into()?;
        Self::new(signed_intent, value.notary_signature.into())
    }
}

fn into_scrypto(
    signed_intent: &SignedIntent,
    notary_signature: &NotarySignature,
) -> ScryptoNotarizedTransaction {
    ScryptoNotarizedTransaction {
        signed_intent: signed_intent.clone().into(),
        notary_signature: (*notary_signature).into(),
    }
}

fn compile_notarized_intent_with(
    signed_intent: &SignedIntent,
    notary_signature: &NotarySignature,
) -> Result<CompiledNotarizedIntent> {
    compile_notarized_intent(into_scrypto(signed_intent, notary_signature))
}

#[cfg(test)]
impl NotarizedTransaction {
    /// Utility function which uses `NotarizedTransaction::new(<SignedIntent>, <NotarySignature>)`
    /// and SHOULD return `Err` if `depth > NotarizedTransaction::MAX_SBOR_DEPTH`, which
    /// we can assert in unit tests.
    pub fn test_with_sbor_depth(
        depth: usize,
        network_id: NetworkID,
    ) -> Result<Self> {
        SignedIntent::test_with_sbor_depth(depth, network_id).and_then(
            |signed_intent| {
                Self::new(
                    signed_intent.clone(),
                    Ed25519PrivateKey::sample_alice()
                        .notarize_hash(&signed_intent.hash()),
                )
            },
        )
    }

    pub const MAX_SBOR_DEPTH: usize = SignedIntent::MAX_SBOR_DEPTH - 1;
}

impl HasSampleValues for NotarizedTransaction {
    fn sample() -> Self {
        let private_key = Ed25519PrivateKey::sample_alice();
        let intent = TransactionIntent::sample();

        let signed_intent =
            SignedIntent::new(intent, IntentSignatures::default()).unwrap();

        let signed_intent_hash = signed_intent.hash();

        Self::new(
            signed_intent,
            private_key.notarize_hash(&signed_intent_hash),
        )
        .unwrap()
    }

    // Identical to: https://github.com/radixdlt/radixdlt-scrypto/blob/ff21f24952318387803ae720105eec079afe33f3/transaction/src/model/hash/encoder.rs#L115
    // intent hash: `"60e5617d670e6c8a42ba5f3749f4ff1079f66221f282554ecdda9ad385ecb195"`
    // bech32 encoded   (mainnet): `"txid_rdx1vrjkzlt8pekg5s46tum5na8lzpulvc3p72p92nkdm2dd8p0vkx2syss63y"`
    // bech32 encoded (simulator): `"txid_sim1vrjkzlt8pekg5s46tum5na8lzpulvc3p72p92nkdm2dd8p0vkx2svr7ejr"`
    fn sample_other() -> Self {
        let private_key: Secp256k1PrivateKey =
            ScryptoSecp256k1PrivateKey::from_u64(1).unwrap().into();

        let intent = TransactionIntent::sample_other();
        assert_eq!(intent.transaction_intent_hash().to_string(), "txid_sim1vrjkzlt8pekg5s46tum5na8lzpulvc3p72p92nkdm2dd8p0vkx2svr7ejr");
        let signed_intent =
            SignedIntent::new(intent, IntentSignatures::new(Vec::new()))
                .unwrap();

        let signed_intent_hash = signed_intent.hash();

        Self::new(
            signed_intent,
            private_key.notarize_hash(&signed_intent_hash),
        )
        .unwrap()
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
        let roundtrip =
            |s: SUT| assert_eq!(SUT::decompile(&s.clone().compile()), s);
        roundtrip(SUT::sample());
        roundtrip(SUT::sample_other())
    }

    #[test]
    fn test_compile() {
        assert_eq!(SUT::sample().compile().to_string(), "4d22030221022104210707010a872c0100000000000a912c01000000000009092f2400220101200720ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf01010800002022044103800051c9a978fb5bfa066a3e5658251ee3304fb9bf58c35b61f8c10e0e7b91840c086c6f636b5f6665652101850000fda0c4277708000000000000000000000000000000004103800051c9a978fb5bfa066a3e5658251ee3304fb9bf58c35b61f8c10e0e7b91840c087769746864726177210280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a48000000000000000000000000000000000280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a480000000000000000000000000000004103800051ac224ee242c339b5ea5f1ae567f0520a6ffa24b52a10b8e6cd96a8347f0c147472795f6465706f7369745f6f725f61626f72742102810000000022000020200022010121020c0a746578742f706c61696e2200010c0c48656c6c6f20526164697821202200220101210120074065938bf04b155de7277d95582ef2f5d36f7200765ee730cf3658da1861ad6e5008df90ac53d2835a48a5c0cb58891297761bda9533411e7eeddb1557d5dbe30a")
    }

    #[test]
    fn notarized_transaction_with_max_sbor_depth_is_ok() {
        assert!(SUT::test_with_sbor_depth(
            SUT::MAX_SBOR_DEPTH,
            NetworkID::Stokenet
        )
        .is_ok());
    }

    #[test]
    fn notarized_transaction_with_sbor_depth_greater_than_max_is_err() {
        assert_eq!(
            SUT::test_with_sbor_depth(
                SUT::MAX_SBOR_DEPTH + 1,
                NetworkID::Stokenet
            ),
            Err(CommonError::InvalidTransactionMaxSBORDepthExceeded {
                max: 24_u16
            })
        );
    }
}
