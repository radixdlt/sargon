use crate::prelude::*;
use sargon::BIP39Passphrase as InternalBIP39Passphrase;

uniffi::custom_newtype!(BIP39Passphrase, String);

/// A BIP39 passphrase, which required but when not used by user, the Default value will be use (empty string),
/// as per BIP39 standard.
#[derive(Debug, Clone, PartialEq, Eq, Hash, InternalConversion)]
pub struct BIP39Passphrase(pub String);
