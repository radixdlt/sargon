use crate::prelude::*;

/// The KDF algorithm used to derive the decryption key from a user provided password.
#[repr(u32)]
#[derive(
    Serialize_repr,
    Deserialize_repr,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::Debug,
)]
pub enum PasswordBasedKeyDerivationSchemeVersion {
    /// A simple password based key derivation algorithm using HKDF<SHA256> with no salt or info.
    /// Description: `"HKDFSHA256-with-UTF8-encoding-of-password-no-salt-no-info"`
    Version1 = 1,
}
