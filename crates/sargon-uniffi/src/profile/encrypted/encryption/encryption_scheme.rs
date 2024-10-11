use crate::prelude::*;
use sargon::EncryptionScheme as InternalEncryptionScheme;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum EncryptionScheme {
    /// AES GCM 256 encryption
    Version1(AesGcm256),
}

impl From<InternalEncryptionScheme> for EncryptionScheme {
    fn from(value: InternalEncryptionScheme) -> Self {
        match value {
            InternalEncryptionScheme::Version1(value) => {
                EncryptionScheme::Version1(value.into())
            }
        }
    }
}

impl Into<InternalEncryptionScheme> for EncryptionScheme {
    fn into(self) -> InternalEncryptionScheme {
        match self {
            EncryptionScheme::Version1(value) => {
                InternalEncryptionScheme::Version1(value.into())
            }
        }
    }
}
