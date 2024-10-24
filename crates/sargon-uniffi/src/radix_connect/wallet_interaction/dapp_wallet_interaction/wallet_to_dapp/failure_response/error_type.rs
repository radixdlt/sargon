use crate::prelude::*;
use sargon::DappWalletInteractionErrorType as InternalDappWalletInteractionErrorType;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Enum)]
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
    InvalidPersonaOrAccounts,
}
