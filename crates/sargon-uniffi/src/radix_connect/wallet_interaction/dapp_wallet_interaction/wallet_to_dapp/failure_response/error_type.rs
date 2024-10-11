use crate::prelude::*;
use sargon::DappWalletInteractionErrorType as InternalDappWalletInteractionErrorType;

#[derive(Clone, PartialEq, uniffi::Enum)]
pub enum DappWalletInteractionErrorType {
    RejectedByUser,
    WrongNetwork,
    FailedToPrepareTransaction,
    FailedToCompileTransaction,
    FailedToSignTransaction,
    FailedToSubmitTransaction,
    FailedToPollSubmittedTransaction,
    FailedToFindAccountWithEnoughFundsToLockFee,
    SubmittedTransactionWasDuplicate,
    SubmittedTransactionHasFailedTransactionStatus,
    SubmittedTransactionHasRejectedTransactionStatus,
    WrongAccountType,
    UnknownWebsite,
    InvalidOriginURL,
    RadixJsonNotFound,
    RadixJsonUnknownFileFormat,
    UnknownDappDefinitionAddress,
    InvalidPersona,
    InvalidRequest,
    IncompatibleVersion,
    FailedToSignAuthChallenge,
}

impl From<InternalDappWalletInteractionErrorType>
    for DappWalletInteractionErrorType
{
    fn from(value: InternalDappWalletInteractionErrorType) -> Self {
        match value {
            InternalDappWalletInteractionErrorType::RejectedByUser => Self::RejectedByUser,
            InternalDappWalletInteractionErrorType::WrongNetwork => Self::WrongNetwork,
            InternalDappWalletInteractionErrorType::FailedToPrepareTransaction => Self::FailedToPrepareTransaction,
            InternalDappWalletInteractionErrorType::FailedToCompileTransaction => Self::FailedToCompileTransaction,
            InternalDappWalletInteractionErrorType::FailedToSignTransaction => Self::FailedToSignTransaction,
            InternalDappWalletInteractionErrorType::FailedToSubmitTransaction => Self::FailedToSubmitTransaction,
            InternalDappWalletInteractionErrorType::FailedToPollSubmittedTransaction => Self::FailedToPollSubmittedTransaction,
            InternalDappWalletInteractionErrorType::FailedToFindAccountWithEnoughFundsToLockFee => Self::FailedToFindAccountWithEnoughFundsToLockFee,
            InternalDappWalletInteractionErrorType::SubmittedTransactionWasDuplicate => Self::SubmittedTransactionWasDuplicate,
            InternalDappWalletInteractionErrorType::SubmittedTransactionHasFailedTransactionStatus => Self::SubmittedTransactionHasFailedTransactionStatus,
            InternalDappWalletInteractionErrorType::SubmittedTransactionHasRejectedTransactionStatus => Self::SubmittedTransactionHasRejectedTransactionStatus,
            InternalDappWalletInteractionErrorType::WrongAccountType => Self::WrongAccountType,
            InternalDappWalletInteractionErrorType::UnknownWebsite => Self::UnknownWebsite,
            InternalDappWalletInteractionErrorType::InvalidOriginURL => Self::InvalidOriginURL,
            InternalDappWalletInteractionErrorType::RadixJsonNotFound => Self::RadixJsonNotFound,
            InternalDappWalletInteractionErrorType::RadixJsonUnknownFileFormat => Self::RadixJsonUnknownFileFormat,
            InternalDappWalletInteractionErrorType::UnknownDappDefinitionAddress => Self::UnknownDappDefinitionAddress,
            InternalDappWalletInteractionErrorType::InvalidPersona => Self::InvalidPersona,
            InternalDappWalletInteractionErrorType::InvalidRequest => Self::InvalidRequest,
            InternalDappWalletInteractionErrorType::IncompatibleVersion => Self::IncompatibleVersion,
            InternalDappWalletInteractionErrorType::FailedToSignAuthChallenge => Self::FailedToSignAuthChallenge,
        }
    }
}

impl Into<InternalDappWalletInteractionErrorType>
    for DappWalletInteractionErrorType
{
    fn into(self) -> InternalDappWalletInteractionErrorType {
        match self {
            Self::RejectedByUser => InternalDappWalletInteractionErrorType::RejectedByUser,
            Self::WrongNetwork => InternalDappWalletInteractionErrorType::WrongNetwork,
            Self::FailedToPrepareTransaction => InternalDappWalletInteractionErrorType::FailedToPrepareTransaction,
            Self::FailedToCompileTransaction => InternalDappWalletInteractionErrorType::FailedToCompileTransaction,
            Self::FailedToSignTransaction => InternalDappWalletInteractionErrorType::FailedToSignTransaction,
            Self::FailedToSubmitTransaction => InternalDappWalletInteractionErrorType::FailedToSubmitTransaction,
            Self::FailedToPollSubmittedTransaction => InternalDappWalletInteractionErrorType::FailedToPollSubmittedTransaction,
            Self::FailedToFindAccountWithEnoughFundsToLockFee => InternalDappWalletInteractionErrorType::FailedToFindAccountWithEnoughFundsToLockFee,
            Self::SubmittedTransactionWasDuplicate => InternalDappWalletInteractionErrorType::SubmittedTransactionWasDuplicate,
            Self::SubmittedTransactionHasFailedTransactionStatus => InternalDappWalletInteractionErrorType::SubmittedTransactionHasFailedTransactionStatus,
            Self::SubmittedTransactionHasRejectedTransactionStatus => InternalDappWalletInteractionErrorType::SubmittedTransactionHasRejectedTransactionStatus,
            Self::WrongAccountType => InternalDappWalletInteractionErrorType::WrongAccountType,
            Self::UnknownWebsite => InternalDappWalletInteractionErrorType::UnknownWebsite,
            Self::InvalidOriginURL => InternalDappWalletInteractionErrorType::InvalidOriginURL,
            Self::RadixJsonNotFound => InternalDappWalletInteractionErrorType::RadixJsonNotFound,
            Self::RadixJsonUnknownFileFormat => InternalDappWalletInteractionErrorType::RadixJsonUnknownFileFormat,
            Self::UnknownDappDefinitionAddress => InternalDappWalletInteractionErrorType::UnknownDappDefinitionAddress,
            Self::InvalidPersona => InternalDappWalletInteractionErrorType::InvalidPersona,
            Self::InvalidRequest => InternalDappWalletInteractionErrorType::InvalidRequest,
            Self::IncompatibleVersion => InternalDappWalletInteractionErrorType::IncompatibleVersion,
            Self::FailedToSignAuthChallenge => InternalDappWalletInteractionErrorType::FailedToSignAuthChallenge,
        }
    }
}
