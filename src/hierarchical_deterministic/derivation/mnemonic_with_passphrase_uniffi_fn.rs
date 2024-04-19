use crate::prelude::*;

json_data_convertible!(MnemonicWithPassphrase);

#[uniffi::export]
pub fn new_mnemonic_with_passphrase_sample() -> MnemonicWithPassphrase {
    MnemonicWithPassphrase::sample()
}

#[uniffi::export]
pub fn new_mnemonic_with_passphrase_sample_other() -> MnemonicWithPassphrase {
    MnemonicWithPassphrase::sample_other()
}

#[cfg(test)]
mod uniffi_test {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = MnemonicWithPassphrase;

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
