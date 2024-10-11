use crate::prelude::*;
use sargon::EncryptionScheme as InternalEncryptionScheme;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
pub enum EncryptionScheme {
    /// AES GCM 256 encryption
    Version1(AesGcm256),
}
