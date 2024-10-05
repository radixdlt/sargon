use crate::prelude::*;
use sargon::MnemonicWithPassphrase as InternalMnemonicWithPassphrase;

/// A BIP39 Mnemonic and BIP39 passphrase - aka "25th word" tuple,
/// from which we can derive a HD Root used for derivation.
#[derive(
    Zeroize,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Debug,
    derive_more::Display,
    uniffi::Record,
)]
#[display("<OBFUSCATED>")]
#[debug("{:?}", self.partially_obfuscated_string())]
pub struct MnemonicWithPassphrase {
    pub mnemonic: Mnemonic,
    pub passphrase: BIP39Passphrase,
}

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

#[uniffi::export]
pub fn mnemonic_with_passphrase_derive_public_keys(
    mnemonic_with_passphrase: &MnemonicWithPassphrase,
    derivation_paths: Vec<DerivationPath>,
) -> Vec<HierarchicalDeterministicPublicKey> {
    mnemonic_with_passphrase.derive_public_keys(derivation_paths)
}

#[uniffi::export]
pub fn mnemonic_with_passphrase_sign(
    mnemonic_with_passphrase: &MnemonicWithPassphrase,
    derivation_path: &DerivationPath,
    hash_to_sign: &Hash,
) -> SignatureWithPublicKey {
    mnemonic_with_passphrase.sign(hash_to_sign, derivation_path)
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

    #[test]
    fn derive_public_keys() {
        assert_eq!(
            mnemonic_with_passphrase_derive_public_keys(
                &SUT::sample(),
                vec![DerivationPath::sample()]
            ),
            vec![HierarchicalDeterministicPublicKey::sample()]
        );
    }

    #[test]
    fn sign() {
        let sut = SUT::sample();
        let path = DerivationPath::sample();
        let key = sut
            .derive_public_keys([path.clone()])
            .into_iter()
            .last()
            .unwrap();
        let msg = Hash::sample();
        let signature = mnemonic_with_passphrase_sign(&sut, &path, &msg);
        assert!(key.public_key.is_valid_signature_for_hash(signature, &msg));
    }
}
