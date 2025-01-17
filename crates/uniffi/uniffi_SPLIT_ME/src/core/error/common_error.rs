use std::fmt::{Display, Formatter};

use crate::prelude::*;
use sargon::CommonError as InternalCommonError;

use thiserror::Error as ThisError;

#[derive(Clone, Debug, ThisError, PartialEq, uniffi::Error)]
pub enum CommonError {
    ErasedError {
        internal_error_code: u32,
        error_message: String,
    },
    SecureStorageAccessError {
        key: String,
        error_kind: SecureStorageAccessErrorKind,
        error_message: String,
    },
    InvalidISO8601String {
        bad_value: String,
    },
    SigningRejected,
    WrongEntityKind {
        expected: String,
        found: String,
    },
    NetworkRequestGenericFailure {
        underlying: String,
    },
    InvalidURL {
        bad_value: String,
    },
    UnknownNetworkForID {
        bad_value: u8,
    },
    FailedToDeserializeJSONToValue {
        json_byte_count: u64,
        type_name: String,
        serde_message: String,
    },
    InvalidSecp256k1PublicKeyPointNotOnCurve,
    InvalidBIP39WordCount {
        bad_value: u64,
    },
    Unknown,
    FileAlreadyExists {
        path: String,
    },
    SecureStorageReadError,
    SecureStorageWriteError,
    UnsafeStorageReadError,
    UnsafeStorageWriteError,
    FailedToDecodeAddressFromBech32 {
        bad_value: String,
    },
    InvalidAppearanceID {
        bad_value: u8,
    },
    DecimalError,
    InvalidByteCount {
        expected: u64,
        found: u64,
    },
    IndexNotHardened {
        bad_value: u32,
    },
    UnknownNetworkID {
        bad_value: u8,
    },
    TooManyBytes {
        max: u64,
        found: u64,
    },
    BytesEmpty,
    FactorOutcomeSignedFactorSourceIDMismatch,
    UnknownAccount,
    NotPermissionToAccessFile {
        path: String,
    },
}

impl Display for CommonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CommonError::ErasedError { error_message, .. } => {
                write!(f, "{}", error_message)
            }
            _ => Display::fmt(&self.into_internal(), f),
        }
    }
}

#[uniffi::export]
pub fn error_message_from_error(error: &CommonError) -> String {
    error.to_string()
}

impl CommonError {
    pub fn error_code(&self) -> u32 {
        match self {
            CommonError::ErasedError {
                internal_error_code,
                ..
            } => *internal_error_code,
            _ => self.into_internal().error_code(),
        }
    }

    pub fn is_safe_to_show_error_message(&self) -> bool {
        matches!(self, CommonError::FailedToDeserializeJSONToValue { .. })
    }
}

#[uniffi::export]
pub fn error_code_from_error(error: &CommonError) -> u32 {
    error.error_code()
}

#[uniffi::export]
pub fn is_safe_to_show_error_message_from_error(error: &CommonError) -> bool {
    error.is_safe_to_show_error_message()
}

impl CommonError {
    fn erased(internal: InternalCommonError) -> Self {
        Self::ErasedError {
            internal_error_code: internal.error_code(),
            error_message: internal.to_string(),
        }
    }
}

impl CommonError {
    pub fn into_internal(&self) -> InternalCommonError {
        match self {
            CommonError::SecureStorageAccessError {
                key,
                error_kind,
                error_message,
            } => InternalCommonError::SecureStorageAccessError {
                key: key.clone(),
                error_kind: error_kind.into_internal(),
                error_message: error_message.clone(),
            },
            CommonError::InvalidISO8601String { bad_value } => {
                InternalCommonError::InvalidISO8601String {
                    bad_value: bad_value.clone(),
                }
            }
            CommonError::SigningRejected => {
                InternalCommonError::SigningRejected
            }
            CommonError::WrongEntityKind { expected, found } => {
                InternalCommonError::WrongEntityKind {
                    expected: expected.clone(),
                    found: found.clone(),
                }
            }
            CommonError::NetworkRequestGenericFailure { underlying } => {
                InternalCommonError::NetworkRequestGenericFailure {
                    underlying: underlying.clone(),
                }
            }
            CommonError::InvalidURL { bad_value } => {
                InternalCommonError::InvalidURL {
                    bad_value: bad_value.clone(),
                }
            }
            CommonError::UnknownNetworkForID { bad_value } => {
                InternalCommonError::UnknownNetworkForID {
                    bad_value: *bad_value,
                }
            }
            CommonError::FailedToDeserializeJSONToValue {
                json_byte_count,
                type_name,
                serde_message,
            } => InternalCommonError::FailedToDeserializeJSONToValue {
                json_byte_count: *json_byte_count,
                type_name: type_name.clone(),
                serde_message: serde_message.clone(),
            },
            CommonError::InvalidSecp256k1PublicKeyPointNotOnCurve => {
                InternalCommonError::InvalidSecp256k1PublicKeyPointNotOnCurve
            }
            CommonError::InvalidBIP39WordCount { bad_value } => {
                InternalCommonError::InvalidBIP39WordCount {
                    bad_value: bad_value.clone(),
                }
            }
            CommonError::Unknown => InternalCommonError::Unknown,
            CommonError::FileAlreadyExists { path } => {
                InternalCommonError::FileAlreadyExists { path: path.clone() }
            }
            CommonError::SecureStorageReadError => {
                InternalCommonError::SecureStorageReadError
            }
            CommonError::SecureStorageWriteError => {
                InternalCommonError::SecureStorageWriteError
            }
            CommonError::UnsafeStorageReadError => {
                InternalCommonError::UnsafeStorageReadError
            }
            CommonError::UnsafeStorageWriteError => {
                InternalCommonError::UnsafeStorageWriteError
            }
            CommonError::FailedToDecodeAddressFromBech32 { bad_value } => {
                InternalCommonError::FailedToDecodeAddressFromBech32 {
                    bad_value: bad_value.clone(),
                }
            }
            CommonError::InvalidAppearanceID { bad_value } => {
                InternalCommonError::InvalidAppearanceID {
                    bad_value: *bad_value,
                }
            }
            CommonError::DecimalError => InternalCommonError::DecimalError,
            CommonError::InvalidByteCount { expected, found } => {
                InternalCommonError::InvalidByteCount {
                    expected: *expected,
                    found: *found,
                }
            }
            CommonError::IndexNotHardened { bad_value } => {
                InternalCommonError::IndexNotHardened {
                    bad_value: *bad_value,
                }
            }
            CommonError::UnknownNetworkID { bad_value } => {
                InternalCommonError::UnknownNetworkID {
                    bad_value: *bad_value,
                }
            }
            CommonError::TooManyBytes { max, found } => {
                InternalCommonError::TooManyBytes {
                    max: *max,
                    found: *found,
                }
            }
            CommonError::BytesEmpty => InternalCommonError::BytesEmpty,
            CommonError::FactorOutcomeSignedFactorSourceIDMismatch => {
                InternalCommonError::FactorOutcomeSignedFactorSourceIDMismatch
            }
            CommonError::UnknownAccount => InternalCommonError::UnknownAccount,
            CommonError::NotPermissionToAccessFile { path } => {
                InternalCommonError::NotPermissionToAccessFile {
                    path: path.clone(),
                }
            }
            _ => InternalCommonError::Unknown,
        }
    }
}

impl Into<InternalCommonError> for CommonError {
    fn into(self) -> InternalCommonError {
        self.into_internal()
    }
}

impl From<InternalCommonError> for CommonError {
    fn from(value: InternalCommonError) -> Self {
        match value {
            InternalCommonError::SecureStorageAccessError {
                key,
                error_kind,
                error_message,
            } => CommonError::SecureStorageAccessError {
                key: key,
                error_kind: error_kind.into(),
                error_message: error_message,
            },
            InternalCommonError::InvalidISO8601String { bad_value } => {
                CommonError::InvalidISO8601String {
                    bad_value: bad_value,
                }
            }
            InternalCommonError::SigningRejected => {
                CommonError::SigningRejected
            }
            InternalCommonError::WrongEntityKind { expected, found } => {
                CommonError::WrongEntityKind {
                    expected: expected,
                    found: found,
                }
            }
            InternalCommonError::NetworkRequestGenericFailure {
                underlying,
            } => CommonError::NetworkRequestGenericFailure {
                underlying: underlying,
            },
            InternalCommonError::InvalidURL { bad_value } => {
                CommonError::InvalidURL {
                    bad_value: bad_value,
                }
            }
            InternalCommonError::UnknownNetworkForID { bad_value } => {
                CommonError::UnknownNetworkForID {
                    bad_value: bad_value,
                }
            }
            InternalCommonError::FailedToDeserializeJSONToValue {
                json_byte_count,
                type_name,
                serde_message,
            } => CommonError::FailedToDeserializeJSONToValue {
                json_byte_count: json_byte_count,
                type_name: type_name,
                serde_message: serde_message,
            },
            InternalCommonError::InvalidSecp256k1PublicKeyPointNotOnCurve => {
                CommonError::InvalidSecp256k1PublicKeyPointNotOnCurve
            }
            InternalCommonError::InvalidBIP39WordCount { bad_value } => {
                CommonError::InvalidBIP39WordCount {
                    bad_value: bad_value,
                }
            }
            InternalCommonError::Unknown => CommonError::Unknown,
            InternalCommonError::FileAlreadyExists { path } => {
                CommonError::FileAlreadyExists { path: path }
            }
            InternalCommonError::SecureStorageReadError => {
                CommonError::SecureStorageReadError
            }
            InternalCommonError::SecureStorageWriteError => {
                CommonError::SecureStorageWriteError
            }
            InternalCommonError::UnsafeStorageReadError => {
                CommonError::UnsafeStorageReadError
            }
            InternalCommonError::UnsafeStorageWriteError => {
                CommonError::UnsafeStorageWriteError
            }
            InternalCommonError::FailedToDecodeAddressFromBech32 {
                bad_value,
            } => CommonError::FailedToDecodeAddressFromBech32 {
                bad_value: bad_value,
            },
            InternalCommonError::InvalidAppearanceID { bad_value } => {
                CommonError::InvalidAppearanceID {
                    bad_value: bad_value,
                }
            }
            InternalCommonError::DecimalError => CommonError::DecimalError,
            InternalCommonError::InvalidByteCount { expected, found } => {
                CommonError::InvalidByteCount {
                    expected: expected,
                    found: found,
                }
            }
            InternalCommonError::IndexNotHardened { bad_value } => {
                CommonError::IndexNotHardened {
                    bad_value: bad_value,
                }
            }
            InternalCommonError::UnknownNetworkID { bad_value } => {
                CommonError::UnknownNetworkID {
                    bad_value: bad_value,
                }
            }
            InternalCommonError::TooManyBytes { max, found } => {
                CommonError::TooManyBytes {
                    max: max,
                    found: found,
                }
            }
            InternalCommonError::BytesEmpty => CommonError::BytesEmpty,
            InternalCommonError::FactorOutcomeSignedFactorSourceIDMismatch => {
                CommonError::FactorOutcomeSignedFactorSourceIDMismatch
            }
            InternalCommonError::UnknownAccount => CommonError::UnknownAccount,
            InternalCommonError::NotPermissionToAccessFile { path } => {
                CommonError::NotPermissionToAccessFile { path: path }
            }
            _ => CommonError::erased(value),
        }
    }
}
