use std::fmt::{Display, Formatter};

use crate::prelude::*;
use sargon::CommonError as InternalCommonError;
use CommonError::*;

use thiserror::Error as ThisError;

#[derive(Clone, Debug, ThisError, PartialEq, uniffi::Error)]
pub enum CommonError {
    // Erased internal errors which we don't have to explicitly expose to the hosts.
    ErasedError {
        internal_error_code: u32,
        error_message: String,
    },

    // Explicit internal errors that are forwarded to the host.
    SecureStorageAccessError {
        key: String,
        error_kind: SecureStorageAccessErrorKind,
        error_message: String,
    },
    InvalidISO8601String {
        bad_value: String,
    },
    HostInteractionAborted,
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
    ReservedInstructionsNotAllowedInManifest {
        reserved_instructions: String,
    },
    OneOfReceivingAccountsDoesNotAllowDeposits,
    FailedTransactionPreview {
        error_message: String,
    },
    FailedToExtractTransactionReceiptBytes,
    MaxTransfersPerTransactionReached {
        amount: u64,
    },
    UnknownNetworkWithName {
        bad_value: String,
    },
    InvalidEd25519PublicKeyFromBytes {
        bad_value: String,
    },
    InvalidSecp256k1PublicKeyFromBytes {
        bad_value: String,
    },
    SigningFailedTooManyFactorSourcesNeglected,
    GatewaySubmitDuplicateTX {
        intent_hash: String,
    },
    UnableToLoadMnemonicFromSecureStorage {
        bad_value: String,
    },
    ExecutionSummaryFail {
        underlying: String,
    },
    FailedToGenerateManifestSummary {
        underlying: String,
    },
    InvalidInstructionsString {
        underlying: String,
    },
    AddressInvalidEntityType {
        address_kind: String,
        entity_type: u8,
        node_id_as_hex: String,
    },
    FailedToFindNetworkIdFromBech32mString {
        bech32m_encoded_address: String,
    },
    InvalidMnemonicWords {
        indices_in_mnemonic: Vec<u8>,
    },
}

#[uniffi::export]
pub fn error_message_from_error(error: &CommonError) -> String {
    error.to_string()
}

#[uniffi::export]
pub fn error_code_from_error(error: &CommonError) -> u32 {
    error.error_code()
}

#[uniffi::export]
pub fn is_safe_to_show_error_message_from_error(error: &CommonError) -> bool {
    error.is_safe_to_show_error_message()
}

impl Display for CommonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErasedError { error_message, .. } => {
                write!(f, "{}", error_message)
            }
            _ => Display::fmt(&self.into_internal(), f),
        }
    }
}

impl CommonError {
    pub fn error_code(&self) -> u32 {
        match self {
            ErasedError {
                internal_error_code,
                ..
            } => *internal_error_code,
            _ => self.into_internal().error_code(),
        }
    }

    pub fn is_safe_to_show_error_message(&self) -> bool {
        matches!(self, FailedToDeserializeJSONToValue { .. })
    }
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
            SecureStorageAccessError {
                key,
                error_kind,
                error_message,
            } => InternalCommonError::SecureStorageAccessError {
                key: key.clone(),
                error_kind: error_kind.into_internal(),
                error_message: error_message.clone(),
            },
            InvalidISO8601String { bad_value } => {
                InternalCommonError::InvalidISO8601String {
                    bad_value: bad_value.clone(),
                }
            }
            HostInteractionAborted => {
                InternalCommonError::HostInteractionAborted
            }
            WrongEntityKind { expected, found } => {
                InternalCommonError::WrongEntityKind {
                    expected: expected.clone(),
                    found: found.clone(),
                }
            }
            NetworkRequestGenericFailure { underlying } => {
                InternalCommonError::NetworkRequestGenericFailure {
                    underlying: underlying.clone(),
                }
            }
            InvalidURL { bad_value } => InternalCommonError::InvalidURL {
                bad_value: bad_value.clone(),
            },
            UnknownNetworkForID { bad_value } => {
                InternalCommonError::UnknownNetworkForID {
                    bad_value: *bad_value,
                }
            }
            FailedToDeserializeJSONToValue {
                json_byte_count,
                type_name,
                serde_message,
            } => InternalCommonError::FailedToDeserializeJSONToValue {
                json_byte_count: *json_byte_count,
                type_name: type_name.clone(),
                serde_message: serde_message.clone(),
            },
            InvalidSecp256k1PublicKeyPointNotOnCurve => {
                InternalCommonError::InvalidSecp256k1PublicKeyPointNotOnCurve
            }
            InvalidBIP39WordCount { bad_value } => {
                InternalCommonError::InvalidBIP39WordCount {
                    bad_value: *bad_value,
                }
            }
            Unknown => InternalCommonError::Unknown,
            FileAlreadyExists { path } => {
                InternalCommonError::FileAlreadyExists { path: path.clone() }
            }
            SecureStorageReadError => {
                InternalCommonError::SecureStorageReadError
            }
            SecureStorageWriteError => {
                InternalCommonError::SecureStorageWriteError
            }
            UnsafeStorageReadError => {
                InternalCommonError::UnsafeStorageReadError
            }
            UnsafeStorageWriteError => {
                InternalCommonError::UnsafeStorageWriteError
            }
            FailedToDecodeAddressFromBech32 { bad_value } => {
                InternalCommonError::FailedToDecodeAddressFromBech32 {
                    bad_value: bad_value.clone(),
                }
            }
            InvalidAppearanceID { bad_value } => {
                InternalCommonError::InvalidAppearanceID {
                    bad_value: *bad_value,
                }
            }
            DecimalError => InternalCommonError::DecimalError,
            InvalidByteCount { expected, found } => {
                InternalCommonError::InvalidByteCount {
                    expected: *expected,
                    found: *found,
                }
            }
            IndexNotHardened { bad_value } => {
                InternalCommonError::IndexNotHardened {
                    bad_value: *bad_value,
                }
            }
            UnknownNetworkID { bad_value } => {
                InternalCommonError::UnknownNetworkID {
                    bad_value: *bad_value,
                }
            }
            TooManyBytes { max, found } => InternalCommonError::TooManyBytes {
                max: *max,
                found: *found,
            },
            BytesEmpty => InternalCommonError::BytesEmpty,
            FactorOutcomeSignedFactorSourceIDMismatch => {
                InternalCommonError::FactorOutcomeSignedFactorSourceIDMismatch
            }
            UnknownAccount => InternalCommonError::UnknownAccount,
            NotPermissionToAccessFile { path } => {
                InternalCommonError::NotPermissionToAccessFile {
                    path: path.clone(),
                }
            }
            ReservedInstructionsNotAllowedInManifest {
                reserved_instructions,
            } => {
                InternalCommonError::ReservedInstructionsNotAllowedInManifest {
                    reserved_instructions: reserved_instructions.clone(),
                }
            }
            OneOfReceivingAccountsDoesNotAllowDeposits => {
                InternalCommonError::OneOfReceivingAccountsDoesNotAllowDeposits
            }
            FailedTransactionPreview { error_message } => {
                InternalCommonError::FailedTransactionPreview {
                    error_message: error_message.clone(),
                }
            }
            FailedToExtractTransactionReceiptBytes => {
                InternalCommonError::FailedToExtractTransactionReceiptBytes
            }
            MaxTransfersPerTransactionReached { amount } => {
                InternalCommonError::MaxTransfersPerTransactionReached {
                    amount: *amount,
                }
            }
            UnknownNetworkWithName { bad_value } => {
                InternalCommonError::UnknownNetworkWithName {
                    bad_value: bad_value.clone(),
                }
            }
            InvalidEd25519PublicKeyFromBytes { bad_value } => {
                InternalCommonError::InvalidEd25519PublicKeyFromBytes {
                    bad_value: bad_value.clone(),
                }
            }
            InvalidSecp256k1PublicKeyFromBytes { bad_value } => {
                InternalCommonError::InvalidSecp256k1PublicKeyFromBytes {
                    bad_value: bad_value.clone(),
                }
            }
            SigningFailedTooManyFactorSourcesNeglected => {
                InternalCommonError::SigningFailedTooManyFactorSourcesNeglected
            }
            GatewaySubmitDuplicateTX { intent_hash } => {
                InternalCommonError::GatewaySubmitDuplicateTX {
                    intent_hash: intent_hash.clone(),
                }
            }
            UnableToLoadMnemonicFromSecureStorage { bad_value } => {
                InternalCommonError::UnableToLoadMnemonicFromSecureStorage {
                    bad_value: bad_value.clone(),
                }
            }
            ExecutionSummaryFail { underlying } => {
                InternalCommonError::ExecutionSummaryFail {
                    underlying: underlying.clone(),
                }
            }
            FailedToGenerateManifestSummary { underlying } => {
                InternalCommonError::FailedToGenerateManifestSummary {
                    underlying: underlying.clone(),
                }
            }
            InvalidInstructionsString { underlying } => {
                InternalCommonError::InvalidInstructionsString {
                    underlying: underlying.clone(),
                }
            }
            AddressInvalidEntityType {
                address_kind,
                entity_type,
                node_id_as_hex,
            } => InternalCommonError::AddressInvalidEntityType {
                address_kind: address_kind.clone(),
                entity_type: *entity_type,
                node_id_as_hex: node_id_as_hex.clone(),
            },
            FailedToFindNetworkIdFromBech32mString {
                bech32m_encoded_address,
            } => InternalCommonError::FailedToFindNetworkIdFromBech32mString {
                bech32m_encoded_address: bech32m_encoded_address.clone(),
            },
            InvalidMnemonicWords {
                indices_in_mnemonic,
            } => InternalCommonError::InvalidMnemonicWords {
                indices_in_mnemonic: indices_in_mnemonic
                    .iter()
                    .map(|i| *i as usize)
                    .collect::<Vec<_>>(),
            },
            _ => InternalCommonError::Unknown,
        }
    }
}

impl From<CommonError> for InternalCommonError {
    fn from(val: CommonError) -> Self {
        val.into_internal()
    }
}

impl From<InternalCommonError> for CommonError {
    fn from(value: InternalCommonError) -> Self {
        match value {
            InternalCommonError::SecureStorageAccessError {
                key,
                error_kind,
                error_message,
            } => SecureStorageAccessError {
                key,
                error_kind: error_kind.into(),
                error_message,
            },
            InternalCommonError::InvalidISO8601String { bad_value } => {
                InvalidISO8601String { bad_value }
            }
            InternalCommonError::HostInteractionAborted => {
                HostInteractionAborted
            }
            InternalCommonError::WrongEntityKind { expected, found } => {
                WrongEntityKind { expected, found }
            }
            InternalCommonError::NetworkRequestGenericFailure {
                underlying,
            } => NetworkRequestGenericFailure { underlying },
            InternalCommonError::InvalidURL { bad_value } => {
                InvalidURL { bad_value }
            }
            InternalCommonError::UnknownNetworkForID { bad_value } => {
                UnknownNetworkForID { bad_value }
            }
            InternalCommonError::FailedToDeserializeJSONToValue {
                json_byte_count,
                type_name,
                serde_message,
            } => FailedToDeserializeJSONToValue {
                json_byte_count,
                type_name,
                serde_message,
            },
            InternalCommonError::InvalidSecp256k1PublicKeyPointNotOnCurve => {
                InvalidSecp256k1PublicKeyPointNotOnCurve
            }
            InternalCommonError::InvalidBIP39WordCount { bad_value } => {
                InvalidBIP39WordCount { bad_value }
            }
            InternalCommonError::Unknown => Unknown,
            InternalCommonError::FileAlreadyExists { path } => {
                FileAlreadyExists { path }
            }
            InternalCommonError::SecureStorageReadError => {
                SecureStorageReadError
            }
            InternalCommonError::SecureStorageWriteError => {
                SecureStorageWriteError
            }
            InternalCommonError::UnsafeStorageReadError => {
                UnsafeStorageReadError
            }
            InternalCommonError::UnsafeStorageWriteError => {
                UnsafeStorageWriteError
            }
            InternalCommonError::FailedToDecodeAddressFromBech32 {
                bad_value,
            } => FailedToDecodeAddressFromBech32 { bad_value },
            InternalCommonError::InvalidAppearanceID { bad_value } => {
                InvalidAppearanceID { bad_value }
            }
            InternalCommonError::DecimalError => DecimalError,
            InternalCommonError::InvalidByteCount { expected, found } => {
                InvalidByteCount { expected, found }
            }
            InternalCommonError::IndexNotHardened { bad_value } => {
                IndexNotHardened { bad_value }
            }
            InternalCommonError::UnknownNetworkID { bad_value } => {
                UnknownNetworkID { bad_value }
            }
            InternalCommonError::TooManyBytes { max, found } => {
                TooManyBytes { max, found }
            }
            InternalCommonError::BytesEmpty => BytesEmpty,
            InternalCommonError::FactorOutcomeSignedFactorSourceIDMismatch => {
                FactorOutcomeSignedFactorSourceIDMismatch
            }
            InternalCommonError::UnknownAccount => UnknownAccount,
            InternalCommonError::NotPermissionToAccessFile { path } => {
                NotPermissionToAccessFile { path }
            }
            InternalCommonError::ReservedInstructionsNotAllowedInManifest {
                reserved_instructions,
            } => ReservedInstructionsNotAllowedInManifest {
                reserved_instructions,
            },
            InternalCommonError::OneOfReceivingAccountsDoesNotAllowDeposits => {
                OneOfReceivingAccountsDoesNotAllowDeposits
            }
            InternalCommonError::FailedTransactionPreview { error_message } => {
                FailedTransactionPreview { error_message }
            }
            InternalCommonError::FailedToExtractTransactionReceiptBytes => {
                FailedToExtractTransactionReceiptBytes
            }
            InternalCommonError::MaxTransfersPerTransactionReached {
                amount,
            } => MaxTransfersPerTransactionReached { amount },
            InternalCommonError::UnknownNetworkWithName { bad_value } => {
                UnknownNetworkWithName { bad_value }
            }
            InternalCommonError::InvalidEd25519PublicKeyFromBytes {
                bad_value,
            } => InvalidEd25519PublicKeyFromBytes { bad_value },
            InternalCommonError::InvalidSecp256k1PublicKeyFromBytes {
                bad_value,
            } => InvalidSecp256k1PublicKeyFromBytes { bad_value },
            InternalCommonError::SigningFailedTooManyFactorSourcesNeglected => {
                SigningFailedTooManyFactorSourcesNeglected
            }
            InternalCommonError::GatewaySubmitDuplicateTX { intent_hash } => {
                GatewaySubmitDuplicateTX {
                    intent_hash: intent_hash.clone(),
                }
            }
            InternalCommonError::UnableToLoadMnemonicFromSecureStorage {
                bad_value,
            } => UnableToLoadMnemonicFromSecureStorage {
                bad_value: bad_value.clone(),
            },
            InternalCommonError::ExecutionSummaryFail { underlying } => {
                ExecutionSummaryFail {
                    underlying: underlying.clone(),
                }
            }
            InternalCommonError::FailedToGenerateManifestSummary {
                underlying,
            } => FailedToGenerateManifestSummary {
                underlying: underlying.clone(),
            },
            InternalCommonError::InvalidInstructionsString { underlying } => {
                InvalidInstructionsString {
                    underlying: underlying.clone(),
                }
            }
            InternalCommonError::AddressInvalidEntityType {
                address_kind,
                entity_type,
                node_id_as_hex,
            } => AddressInvalidEntityType {
                address_kind: address_kind.clone(),
                entity_type,
                node_id_as_hex: node_id_as_hex.clone(),
            },
            InternalCommonError::FailedToFindNetworkIdFromBech32mString {
                bech32m_encoded_address,
            } => FailedToFindNetworkIdFromBech32mString {
                bech32m_encoded_address: bech32m_encoded_address.clone(),
            },
            InternalCommonError::InvalidMnemonicWords {
                indices_in_mnemonic,
            } => InvalidMnemonicWords {
                indices_in_mnemonic: indices_in_mnemonic
                    .into_iter()
                    .map(|i| i as u8)
                    .collect::<Vec<_>>(),
            },
            _ => Self::erased(value),
        }
    }
}
