use crate::prelude::*;

/// An encryption of a `ProfileSnapshot` with crypto metadata about how it was encrypted, which can
/// be used to decrypt it, given a user provided password.
#[derive(
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::Debug,
)]
#[display("{}: {}", version, key_derivation_scheme)]
pub struct EncryptedProfileSnapshot {
    /// JSON format version of this struct
    pub version: ProfileEncryptionVersion,

    /// Encrypted JSON encoding of a `ProfileSnapshot`
    #[serde(rename = "encryptedSnapshot")]
    pub encrypted_snapshot: BagOfBytes,

    /// The KDF algorithm which was used to derive the encryption key from the user provided password.
    #[serde(rename = "keyDerivationScheme")]
    pub key_derivation_scheme: PasswordBasedKeyDerivationScheme,

    /// The encryption algorithm which was used to produce `encryptedSnapshot` with the encryption key
    /// derived using the `keyDerivationScheme`.
    #[serde(rename = "encryptionScheme")]
    pub encryption_scheme: EncryptionScheme,
}

#[derive(
    Copy,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    derive_more::Display,
    derive_more::Debug,
    PartialOrd,
    Ord,
    Hash,
)]
#[serde(transparent)]
pub struct ProfileEncryptionVersion(u32);

uniffi::custom_newtype!(ProfileEncryptionVersion, u32);
