use crate::prelude::*;
use sargon::BIP39Passphrase as InternalBIP39Passphrase;

/// A BIP39 passphrase, which required but when not used by user, the Default value will be use (empty string),
/// as per BIP39 standard.
#[derive(
    Zeroize,
    Clone,
    PartialEq,
    Eq,
    Hash,
    uniffi::Record,
)]
pub struct BIP39Passphrase {
    pub value: String
}

impl From<InternalBIP39Passphrase> for BIP39Passphrase {
    fn from(value: InternalBIP39Passphrase) -> Self {
        Self { value: value.0 }
    }
}

impl Into<InternalBIP39Passphrase> for BIP39Passphrase {
    fn into(self) -> InternalBIP39Passphrase {
        InternalBIP39Passphrase(self.value)
    }
}