use crate::prelude::*;
use encryption::EncryptionScheme as InternalEncryptionScheme;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum EncryptionScheme {
    /// AES GCM 256 encryption
    Version1(AesGcm256),
}
