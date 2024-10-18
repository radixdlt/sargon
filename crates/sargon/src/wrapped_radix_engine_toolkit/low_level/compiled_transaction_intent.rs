use crate::prelude::*;

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::FromStr,
)]
pub struct CompiledTransactionIntent(BagOfBytes);

impl CompiledTransactionIntent {
    pub fn new(bytes: BagOfBytes) -> Self {
        Self(bytes)
    }

    pub fn bytes(&self) -> BagOfBytes {
        self.0.clone()
    }

    pub fn decompile(&self) -> TransactionIntent {
        let err = "Should never fail to decompile a 'CompiledTransactionIntent' since we should not have been able to construct an invalid 'CompiledTransactionIntent'.";

        let notarized = RET_decompile_intent(self.bytes()).expect(err);

        notarized.try_into().expect(err)
    }
}

impl TransactionIntent {
    pub fn compile(&self) -> CompiledTransactionIntent {
        let bytes = super::compile_intent(ScryptoIntent::from(self.clone()))
            .expect("Should always be able to compile an Intent");

        CompiledTransactionIntent(bytes)
    }
}

impl HasSampleValues for CompiledTransactionIntent {
    fn sample() -> Self {
        TransactionIntent::sample().compile()
    }

    fn sample_other() -> Self {
        TransactionIntent::sample_other().compile()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = CompiledTransactionIntent;

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
    fn from_str() {
        assert_eq!(
            "4d220104210707010a872c0100000000000a912c01000000000009092f2400220101200720ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf01010800002022044103800051c9a978fb5bfa066a3e5658251ee3304fb9bf58c35b61f8c10e0e7b91840c086c6f636b5f6665652101850000fda0c4277708000000000000000000000000000000004103800051c9a978fb5bfa066a3e5658251ee3304fb9bf58c35b61f8c10e0e7b91840c087769746864726177210280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a48000000000000000000000000000000000280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a480000000000000000000000000000004103800051ac224ee242c339b5ea5f1ae567f0520a6ffa24b52a10b8e6cd96a8347f0c147472795f6465706f7369745f6f725f61626f72742102810000000022000020200022010121020c0a746578742f706c61696e2200010c0c48656c6c6f20526164697821".parse::<SUT>().unwrap(),
            SUT::sample()
        );
    }

    #[test]
    fn to_string() {
        assert_eq!(
            "4d220104210707f20a00000000000000000a0a00000000000000090a0000002200012007210279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f817980101080000202200202000220000",
            SUT::sample_other().to_string(),
        );
    }

    #[test]
    fn decompile() {
        assert_eq!(SUT::sample().decompile(), TransactionIntent::sample());
        assert_eq!(
            SUT::sample_other().decompile(),
            TransactionIntent::sample_other()
        );
    }

    #[test]
    #[should_panic(
        expected = "Should never fail to decompile a 'CompiledTransactionIntent' since we should not have been able to construct an invalid 'CompiledTransactionIntent'."
    )]
    fn decompile_fail() {
        _ = CompiledTransactionIntent::new(BagOfBytes::sample_aced()).decompile();
    }

    #[test]
    fn other_reasons_for_invalid() {
        let res = compile_notarized_intent(ScryptoNotarizedTransaction {
            signed_intent: invalid_signed_intent(),
            notary_signature: NotarySignature::sample().into(),
        });
        assert_eq!(
            res,
            Err(CommonError::InvalidNotarizedIntentFailedToEncode { underlying: "MismatchingArrayElementValueKind { element_value_kind: 7, actual_value_kind: 8 }".to_owned() })
        );
    }
}
