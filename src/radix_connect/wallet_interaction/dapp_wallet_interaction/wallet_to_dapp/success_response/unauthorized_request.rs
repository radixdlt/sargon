use crate::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappWalletInteractionUnauthorizedRequestResponseItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_accounts:
        Option<DappWalletInteractionAccountsRequestResponseItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_persona_data: Option<DappWalletInteractionPersonaDataRequestResponseItem>,
}

impl HasSampleValues for DappWalletInteractionUnauthorizedRequestResponseItems {
    fn sample() -> Self {
        Self {
            one_time_accounts: Some(DappWalletInteractionAccountsRequestResponseItem::sample()),
            one_time_persona_data: Some(DappWalletInteractionPersonaDataRequestResponseItem::sample()),
        }
    }

    fn sample_other() -> Self {
        Self {
            one_time_accounts: Some(DappWalletInteractionAccountsRequestResponseItem::sample_other()),
            one_time_persona_data: Some(DappWalletInteractionPersonaDataRequestResponseItem::sample_other()),
        }
    }
}