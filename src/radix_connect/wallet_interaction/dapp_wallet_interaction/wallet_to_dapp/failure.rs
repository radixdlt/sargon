use crate::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappWalletInteractionFailureResponse {
    pub interaction_id: WalletInteractionId,
    pub error: DappWalletInteractionErrorType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl HasSampleValues for DappWalletInteractionFailureResponse {
    fn sample() -> Self {
        Self {
            interaction_id: WalletInteractionId::sample(),
            error: DappWalletInteractionErrorType::sample(),
            message: Some("sample1".to_string()),
        }
    }

    fn sample_other() -> Self {
        Self {
            interaction_id: WalletInteractionId::sample_other(),
            error: DappWalletInteractionErrorType::sample_other(),
            message: Some("sample2".to_string()),
        }
    }
}

#[derive(Debug, Serialize, PartialEq, uniffi::Enum)]
#[serde(rename_all = "camelCase")]
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

impl HasSampleValues for DappWalletInteractionErrorType {
    fn sample() -> Self {
        DappWalletInteractionErrorType::FailedToPrepareTransaction
    }

    fn sample_other() -> Self {
        DappWalletInteractionErrorType::FailedToCompileTransaction
    }
}