use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash, derive_more::Debug, uniffi::Enum)]
pub enum EncryptionScheme {
    /// AES GCM 256 encryption
    Version1(AesGcm256),
}
