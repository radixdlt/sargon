use crate::prelude::*;
use sargon::MnemonicWithPassphrase as InternalMnemonicWithPassphrase;

/// A BIP39 Mnemonic and BIP39 passphrase - aka "25th word" tuple,
/// from which we can derive a HD Root used for derivation.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct MnemonicWithPassphrase {
    pub mnemonic: Mnemonic,
    pub passphrase: BIP39Passphrase,
}

impl From<InternalMnemonicWithPassphrase> for MnemonicWithPassphrase {
    fn from(value: InternalMnemonicWithPassphrase) -> Self {
        Self {
            mnemonic: value.mnemonic.into(),
            passphrase: value.passphrase.into(),
        }
    }
}

impl Into<InternalMnemonicWithPassphrase> for MnemonicWithPassphrase {
    fn into(self) -> InternalMnemonicWithPassphrase {
        InternalMnemonicWithPassphrase {
            mnemonic: self.mnemonic.into(),
            passphrase: self.passphrase.into(),
        }
    }
}

json_data_convertible!(MnemonicWithPassphrase);

#[uniffi::export]
pub fn new_mnemonic_with_passphrase_sample() -> MnemonicWithPassphrase {
    InternalMnemonicWithPassphrase::sample().into()
}

#[uniffi::export]
pub fn new_mnemonic_with_passphrase_sample_other() -> MnemonicWithPassphrase {
    InternalMnemonicWithPassphrase::sample_other().into()
}

/// Returns `true` if this MnemonicWithPassphrase successfully validates all `hd_keys`, that is to say,
/// that all the HierarchicalDeterministicPublicKey were indeed crated by this MnemonicWithPassphrase.
#[uniffi::export]
pub fn mnemonic_with_passphrase_validate_public_keys(
    mnemonic_with_passphrase: &MnemonicWithPassphrase,
    hd_keys: Vec<HierarchicalDeterministicPublicKey>,
) -> bool {
    mnemonic_with_passphrase
        .into_internal()
        .validate_public_keys(hd_keys.into_internal_vec())
}

#[uniffi::export]
pub fn mnemonic_with_passphrase_derive_public_keys(
    mnemonic_with_passphrase: &MnemonicWithPassphrase,
    derivation_paths: Vec<DerivationPath>,
) -> Vec<HierarchicalDeterministicPublicKey> {
    mnemonic_with_passphrase
        .into_internal()
        .derive_public_keys(derivation_paths.into_internal_vec())
        .into_vec()
}

#[uniffi::export]
pub fn mnemonic_with_passphrase_sign(
    mnemonic_with_passphrase: &MnemonicWithPassphrase,
    derivation_path: &DerivationPath,
    hash_to_sign: &Hash,
) -> SignatureWithPublicKey {
    mnemonic_with_passphrase
        .into_internal()
        .sign(
            &hash_to_sign.into_internal(),
            &derivation_path.into_internal(),
        )
        .into()
}
