use crate::prelude::*;

/// Versioned Encryption scheme
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
pub enum EncryptionSchemeVersion {
    /// AES GCM 256 encryption
    Version1 = 1,
}
