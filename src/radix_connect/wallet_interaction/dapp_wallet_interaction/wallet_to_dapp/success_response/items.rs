use crate::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
#[serde(tag = "discriminator")]
pub enum DappWalletInteractionResponseItems {
    #[serde(rename = "authorizedRequest")]
    AuthorizedRequest(DappWalletInteractionAuthorizedRequestResponseItems),
    #[serde(rename = "unauthorizedRequest")]
    UnauthorizedRequest(DappWalletInteractionUnauthorizedRequestResponseItems),
    #[serde(rename = "transaction")]
    Transaction(DappWalletInteractionTransactionResponseItems),
}
