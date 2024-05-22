use crate::prelude::*;

impl Mnemonic {
    pub fn from_entropy_in(
        entropy: NonEmptyMax32Bytes,
        language: BIP39Language,
    ) -> Result<Self> {
        let internal =
            bip39::Mnemonic::from_entropy_in(language.into(), entropy.as_ref())
                .map_err(|_| CommonError::InvalidMnemonicPhrase)?;

        Ok(Self::from_internal(internal))
    }

    pub fn from_entropy(entropy: NonEmptyMax32Bytes) -> Result<Self> {
        Self::from_entropy_in(entropy, BIP39Language::English)
    }

    pub fn generate_new() -> Self {
        Self::from_entropy(NonEmptyMax32Bytes::generate())
            .expect("Should have generated 32 bytes")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Mnemonic;

    #[test]
    fn mnemonic_from_entropy_of_16_bytes() {
        let sut =
            SUT::from_entropy(NonEmptyMax32Bytes::from([0xff; 16])).unwrap();
        assert_eq!(
            sut.phrase(),
            "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong"
        )
    }

    #[test]
    fn mnemonic_from_entropy_of_20_bytes() {
        let sut =
            SUT::from_entropy(NonEmptyMax32Bytes::from([0xff; 20])).unwrap();
        assert_eq!(
            sut.phrase(),
            "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrist"
        )
    }

    #[test]
    fn mnemonic_from_entropy_of_24_bytes() {
        let sut =
            SUT::from_entropy(NonEmptyMax32Bytes::from([0xff; 24])).unwrap();
        assert_eq!(
            sut.phrase(),
            "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo when"
        )
    }

    #[test]
    fn mnemonic_from_entropy_of_28_bytes() {
        let sut =
            SUT::from_entropy(NonEmptyMax32Bytes::from([0xff; 28])).unwrap();
        assert_eq!(
            sut.phrase(),
            "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo veteran"
        );
    }

    #[test]
    fn mnemonic_from_entropy_of_32_bytes() {
        let sut =
            SUT::from_entropy(NonEmptyMax32Bytes::from([0xff; 32])).unwrap();
        assert_eq!(
            sut.phrase(),
            "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo vote"
        )
    }
}
