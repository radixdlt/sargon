use crate::prelude::*;

#[derive(Debug, Serialize, PartialEq, uniffi::Enum)]
#[serde(tag = "discriminator")]
pub enum WalletToDappInteractionResponseItems {
    #[serde(rename = "authorizedRequest")]
    AuthorizedRequest(WalletToDappInteractionAuthorizedRequestResponseItems),
    #[serde(rename = "unauthorizedRequest")]
    UnauthorizedRequest(WalletToDappInteractionUnauthorizedRequestResponseItems),
    #[serde(rename = "transaction")]
    Transaction(WalletToDappInteractionTransactionResponseItems),
}

impl HasSampleValues for WalletToDappInteractionResponseItems {
    fn sample() -> Self {
        WalletToDappInteractionResponseItems::AuthorizedRequest(
            WalletToDappInteractionAuthorizedRequestResponseItems::sample(),
        )
    }
    fn sample_other() -> Self {
        WalletToDappInteractionResponseItems::Transaction(
            WalletToDappInteractionTransactionResponseItems::sample_other(),
        )
    }
}