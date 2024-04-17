use super::authorized_request::DappToWalletInteractionAuthorizedRequestItems;
use super::transaction::DappToWalletInteractionTransactionItems;
use super::unauthorized_request::DappToWalletInteractionUnauthorizedRequestItems;
use crate::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, uniffi::Enum)]
#[serde(tag = "discriminator")]
pub enum DappToWalletInteractionItems {
    #[serde(rename = "unauthorizedRequest")]
    UnauthorizedRequest(DappToWalletInteractionUnauthorizedRequestItems),
    #[serde(rename = "authorizedRequest")]
    AuthorizedRequest(DappToWalletInteractionAuthorizedRequestItems),
    #[serde(rename = "transaction")]
    Transaction(DappToWalletInteractionTransactionItems),
}

impl HasSampleValues for DappToWalletInteractionItems {
    fn sample() -> Self {
        Self::UnauthorizedRequest(
            DappToWalletInteractionUnauthorizedRequestItems::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::Transaction(DappToWalletInteractionTransactionItems::sample())
    }
}
