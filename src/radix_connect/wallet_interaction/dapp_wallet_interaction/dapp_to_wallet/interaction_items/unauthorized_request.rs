use crate::prelude::*;
use serde::Deserialize;
use super::request::*;

#[derive(Debug, Deserialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionUnauthorizedRequestItems {
    pub one_time_accounts: Option<DappToWalletInteractionAccountsRequestItem>,
    pub one_time_persona_data:
        Option<DappToWalletInteractionPersonaDataRequestItem>,
}

impl HasSampleValues for DappToWalletInteractionUnauthorizedRequestItems {
    fn sample() -> Self {
        Self {
            one_time_accounts: Some(DappToWalletInteractionAccountsRequestItem::sample()),
            one_time_persona_data: Some(DappToWalletInteractionPersonaDataRequestItem::sample()),
        }
    }

    fn sample_other() -> Self {
        Self {
            one_time_accounts: Some(DappToWalletInteractionAccountsRequestItem::sample_other()),
            one_time_persona_data: Some(DappToWalletInteractionPersonaDataRequestItem::sample_other()),
        }
    }
}