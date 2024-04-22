use crate::prelude::*;

#[derive(Debug, Serialize, PartialEq, uniffi::Enum)]
#[serde(tag = "discriminator")]
pub enum DappWalletInteractionResponseItems {
    #[serde(rename = "authorizedRequest")]
    AuthorizedRequest(DappWalletInteractionAuthorizedRequestResponseItems),
    #[serde(rename = "unauthorizedRequest")]
    UnauthorizedRequest(DappWalletInteractionUnauthorizedRequestResponseItems),
    #[serde(rename = "transaction")]
    Transaction(DappWalletInteractionTransactionResponseItems),
}

impl HasSampleValues for DappWalletInteractionResponseItems {
    fn sample() -> Self {
        DappWalletInteractionResponseItems::AuthorizedRequest(
            DappWalletInteractionAuthorizedRequestResponseItems::sample(),
        )
    }
    fn sample_other() -> Self {
        DappWalletInteractionResponseItems::Transaction(
            DappWalletInteractionTransactionResponseItems::sample_other(),
        )
    }
}