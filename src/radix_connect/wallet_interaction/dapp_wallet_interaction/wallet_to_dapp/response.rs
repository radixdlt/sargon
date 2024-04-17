use crate::prelude::*;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize, PartialEq)]
#[serde(tag = "discriminator")]
pub enum DappWalletInteractionResponse {
    #[serde(rename = "success")]
    Success(DappWalletInteractionSuccessResponse),
    #[serde(rename = "failure")]
    Failure(DappWalletInteractionFailureResponse),
}

#[derive(Debug, Serialize, PartialEq)]
pub struct DappWalletInteractionSuccessResponse {
    pub interaction_id: WalletInteractionId,
    pub items: DappWalletInteractionResponseItems,
}

#[derive(Debug, Serialize, PartialEq)]
pub enum DappWalletInteractionResponseItems {
    Request(DappWalletInteractionRequestResponseItems),
    Transaction(DappWalletInteractionTransactionResponseItems),
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(tag = "discriminator")]
pub enum DappWalletInteractionRequestResponseItems {
    Unauthorized(DappWalletInteractionUnauthorizedRequestResponseItems),
    Authorized(DappWalletInteractionUnauthorizedRequestResponseItems),
}

#[derive(Debug, Serialize, PartialEq)]
pub struct DappWalletInteractionUnauthorizedRequestResponseItems {
    pub one_time_accounts:
        Option<DappWalletInteractionAccountsRequestResponseItem>,
    pub one_time_persona_data:
        Option<DappWalletInteractionPersonaDataRequestResponseItem>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct DappWalletInteractionAccountsRequestResponseItem {
    pub accounts: Vec<WalletInteractionWalletAccount>,
    pub challenge: Option<Exactly32Bytes>,
    pub proofs: Option<Vec<DappWalletInteractionAccountProof>>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct WalletInteractionWalletAccount {
    pub address: AccountAddress,
    pub label: String,
    pub appearance_id: AppearanceID,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct DappWalletInteractionAccountProof {
    pub account_address: AccountAddress,
    pub proof: DappWalletInteractionAuthProof,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct DappWalletInteractionAuthProof {
    pub public_key: String,
    pub curve: SLIP10Curve,
    pub signature: String,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct DappWalletInteractionPersonaDataRequestResponseItem {
    pub name: Option<DappWalletInteractionPersonaDataName>,
    pub email_addresses: Option<Vec<DappWalletInteractionPersonaDataEmail>>,
    pub phone_numbers: Option<Vec<DappWalletInteractionPersonaDataPhoneNumber>>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct DappWalletInteractionPersonaDataName {
    pub variant: DappWalletInteractionPersonaDataNameVariant,
    pub family_name: String,
    pub given_names: String,
    pub nickname: String,
}

#[derive(Debug, Serialize, PartialEq)]
pub enum DappWalletInteractionPersonaDataNameVariant {
    Western,
    Eastern,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct DappWalletInteractionPersonaDataEmail(String);

#[derive(Debug, Serialize, PartialEq)]
pub struct DappWalletInteractionPersonaDataPhoneNumber(String);

#[derive(Debug, Serialize, PartialEq)]
pub struct DappWalletInteractionAuthorizedRequestResponseItems {
    pub auth: DappWalletInteractionAuthRequestResponseItem,
    pub ongoing_accounts:
        Option<DappWalletInteractionAccountsRequestResponseItem>,
    pub ongoing_persona_data:
        Option<DappWalletInteractionAuthUsePersonaRequestResponseItem>,
    pub one_time_accounts:
        Option<DappWalletInteractionAccountsRequestResponseItem>,
    pub one_time_persona_data:
        Option<DappWalletInteractionAuthUsePersonaRequestResponseItem>,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(tag = "discriminator")]
pub enum DappWalletInteractionAuthRequestResponseItem {
    #[serde(rename = "usePersona")]
    UsePersona(DappWalletInteractionAuthUsePersonaRequestResponseItem),
    #[serde(rename = "loginWithoutChallenge")]
    LoginWithoutChallenge(
        DappWalletInteractionAuthLoginWithoutChallengeRequestResponseItem,
    ),
    #[serde(rename = "loginWithChallenge")]
    LoginWithChallenge(
        DappWalletInteractionAuthLoginWithChallengeRequestResponseItem,
    ),
}

#[derive(Debug, Serialize, PartialEq)]
pub struct DappWalletInteractionAuthUsePersonaRequestResponseItem {
    pub persona: DappWalletInteractionPersona,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct DappWalletInteractionAuthLoginWithoutChallengeRequestResponseItem {
    pub persona: DappWalletInteractionPersona,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct DappWalletInteractionAuthLoginWithChallengeRequestResponseItem {
    pub persona: DappWalletInteractionPersona,
    pub challenge: Exactly32Bytes,
    pub proof: DappWalletInteractionAuthProof,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct DappWalletInteractionPersona {
    pub identity_address: IdentityAddress,
    pub label: String,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct DappWalletInteractionTransactionResponseItems {
    pub send: DappWalletInteractionSendTransactionResponseItem,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct DappWalletInteractionSendTransactionResponseItem {
    pub transactionIntentHash: String,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct DappWalletInteractionFailureResponse {
    pub interaction_id: WalletInteractionId,
    pub error: DappWalletInteractionErrorType,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, PartialEq)]
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
