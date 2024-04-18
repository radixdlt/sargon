use crate::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappWalletInteractionFailureResponse {
    pub interaction_id: WalletInteractionId,
    pub error: DappWalletInteractionErrorType,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, PartialEq, uniffi::Enum)]
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
