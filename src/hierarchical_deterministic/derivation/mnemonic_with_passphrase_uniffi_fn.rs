use crate::prelude::*;

#[uniffi::export]
pub fn new_mnemonic_with_passphrase_sample() -> MnemonicWithPassphrase {
    MnemonicWithPassphrase::sample()
}

#[uniffi::export]
pub fn new_mnemonic_with_passphrase_sample_other() -> MnemonicWithPassphrase {
    MnemonicWithPassphrase::sample_other()
}

#[uniffi::export]
pub fn new_mnemonic_with_passphrase_from_json_bytes(
    json_bytes: BagOfBytes,
) -> Result<MnemonicWithPassphrase> {
    MnemonicWithPassphrase::new_from_json_bytes(json_bytes)
}

#[uniffi::export]
pub fn mnemonic_with_passphrase_to_json_bytes(
    mnemonic_with_passphrase: &MnemonicWithPassphrase,
) -> BagOfBytes {
    mnemonic_with_passphrase.to_json_bytes().into()
}

#[cfg(test)]
mod uniffi_test {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = MnemonicWithPassphrase;

    #[test]
    fn json_bytes_roundtrip() {
        let sut = SUT::sample();
        let json_bytes = mnemonic_with_passphrase_to_json_bytes(&sut);
        assert_eq!(
            sut,
            new_mnemonic_with_passphrase_from_json_bytes(json_bytes).unwrap()
        );
    }

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_mnemonic_with_passphrase_sample(),
                new_mnemonic_with_passphrase_sample_other(),
                // duplicates should get removed
                new_mnemonic_with_passphrase_sample(),
                new_mnemonic_with_passphrase_sample_other(),
            ])
            .len(),
            2
        );
    }
}
