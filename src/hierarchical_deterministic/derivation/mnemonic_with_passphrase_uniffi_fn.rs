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

/// Returns `true` if this MnemonicWithPassphrase successfully validates all `hd_keys`, that is to say,
/// that all the HierarchicalDeterministicPublicKey were indeed crated by this MnemonicWithPassphrase.
#[uniffi::export]
pub fn mnemonic_with_passphrase_validate_public_keys(
    mnemonic_with_passphrase: &MnemonicWithPassphrase,
    hd_keys: Vec<HierarchicalDeterministicPublicKey>,
) -> bool {
    mnemonic_with_passphrase.validate_public_keys(hd_keys)
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

    #[test]
    fn validate() {
        assert!(!mnemonic_with_passphrase_validate_public_keys(
            &SUT::sample_other(),
            vec![HierarchicalDeterministicPublicKey::sample()]
        ));
    }
}
