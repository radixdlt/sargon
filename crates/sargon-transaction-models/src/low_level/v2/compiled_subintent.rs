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
pub struct CompiledSubintent(BagOfBytes);

impl CompiledSubintent {
    /// Constructs a `CompiledSubintent` from bytes.
    /// Fails if the bytes do not construct a valid `Subintent`
    pub fn new(bytes: BagOfBytes) -> Result<Self> {
        RET_subintent_from_payload_bytes(bytes.clone())
            .map(|_| Self(bytes))
            .map_err(|_| CommonError::FailedToDecompileBytesIntoSubintent)
    }

    pub fn bytes(&self) -> BagOfBytes {
        self.0.clone()
    }

    pub fn decompile(&self) -> Subintent {
        let err = "Should never fail to decompile a 'CompiledSubintent' since we should not have been able to construct an invalid 'Subintent'.";

        let scrypto_subintent =
            RET_subintent_from_payload_bytes(self.bytes()).expect(err);

        scrypto_subintent.try_into().expect(err)
    }
}

impl Subintent {
    pub fn compile(&self) -> CompiledSubintent {
        let bytes = RET_subintent_to_payload_bytes(&ScryptoSubintent::from(
            self.clone(),
        ))
        .expect("Should always be able to compile a Subintent");

        CompiledSubintent(bytes.into())
    }
}

impl From<CompiledSubintent> for SubintentHash {
    fn from(val: CompiledSubintent) -> Self {
        val.decompile().hash()
    }
}

impl From<Subintent> for CompiledSubintent {
    fn from(value: Subintent) -> Self {
        value.compile()
    }
}

impl HasSampleValues for CompiledSubintent {
    fn sample() -> Self {
        Subintent::sample().compile()
    }

    fn sample_other() -> Self {
        Subintent::sample_other().compile()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = CompiledSubintent;

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
            "4d220b012105210607010a872c0100000000000a912c01000000000022010105008306670000000022010105e8860667000000000a15cd5b070000000020200022010121020c0a746578742f706c61696e2200010c0c48656c6c6f205261646978212020002022054103800051c9a978fb5bfa066a3e5658251ee3304fb9bf58c35b61f8c10e0e7b91840c086c6f636b5f6665652101850000fda0c4277708000000000000000000000000000000004103800051c9a978fb5bfa066a3e5658251ee3304fb9bf58c35b61f8c10e0e7b91840c087769746864726177210280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a48000000000000000000000000000000000280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a480000000000000000000000000000004103800051ac224ee242c339b5ea5f1ae567f0520a6ffa24b52a10b8e6cd96a8347f0c147472795f6465706f7369745f6f725f61626f72742102810000000022000060012100".parse::<SUT>().unwrap(),
            SUT::sample()
        );
    }

    #[test]
    fn to_string() {
        assert_eq!(
            "4d220b012105210607f20a00000000000000000a0a000000000000002200002200000ab168de3a00000000202000220000202000202200",
            SUT::sample_other().to_string(),
        );
    }

    #[test]
    fn decompile() {
        assert_eq!(SUT::sample().decompile(), Subintent::sample());
        assert_eq!(SUT::sample_other().decompile(), Subintent::sample_other());
    }

    #[test]
    fn construct_fail() {
        assert_eq!(
            SUT::new(BagOfBytes::sample_aced()),
            Err(CommonError::FailedToDecompileBytesIntoSubintent)
        );
    }
}
