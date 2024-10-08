use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum EncryptionScheme {
    /// AES GCM 256 encryption
    Version1(AesGcm256),
}
