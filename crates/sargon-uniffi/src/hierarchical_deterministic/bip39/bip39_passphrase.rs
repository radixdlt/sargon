use crate::prelude::*;

// Generate the FfiConverter needed by UniFFI for newtype `BIP39Passphrase`.
uniffi::custom_newtype!(BIP39Passphrase, String);

/// A BIP39 passphrase, which required but when not used by user, the Default value will be use (empty string),
/// as per BIP39 standard.
#[derive(
    Zeroize,
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Eq,
    derive_more::Display,
    derive_more::Debug,
    Hash,
)]
#[serde(transparent)]
#[display("<OBFUSCATED>")]
#[debug("{:?}", self.partially_obfuscated_string())]
pub struct BIP39Passphrase(pub String);