use crate::prelude::*;
use serde::Deserialize;
use super::request::*;

#[derive(Debug, Deserialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionAuthorizedRequestItems {
    pub auth: DappToWalletInteractionAuthRequestItem,
    pub reset: Option<DappToWalletInteractionResetRequestItem>,
    pub ongoing_accounts: Option<DappToWalletInteractionAccountsRequestItem>,
    pub ongoing_persona_data:
        Option<DappToWalletInteractionPersonaDataRequestItem>,
    pub one_time_accounts: Option<DappToWalletInteractionAccountsRequestItem>,
    pub one_time_persona_data:
        Option<DappToWalletInteractionPersonaDataRequestItem>,
}

impl HasSampleValues for DappToWalletInteractionAuthorizedRequestItems {
    fn sample() -> Self {
        Self {
            auth: DappToWalletInteractionAuthRequestItem::sample(),
            reset: Some(DappToWalletInteractionResetRequestItem::sample()),
            ongoing_accounts: Some(DappToWalletInteractionAccountsRequestItem::sample()),
            ongoing_persona_data: Some(DappToWalletInteractionPersonaDataRequestItem::sample()),
            one_time_accounts: Some(DappToWalletInteractionAccountsRequestItem::sample()),
            one_time_persona_data: Some(DappToWalletInteractionPersonaDataRequestItem::sample()),
        }
    }

    fn sample_other() -> Self {
        Self {
            auth: DappToWalletInteractionAuthRequestItem::sample_other(),
            reset: Some(DappToWalletInteractionResetRequestItem::sample_other()),
            ongoing_accounts: Some(DappToWalletInteractionAccountsRequestItem::sample_other()),
            ongoing_persona_data: Some(DappToWalletInteractionPersonaDataRequestItem::sample_other()),
            one_time_accounts: Some(DappToWalletInteractionAccountsRequestItem::sample_other()),
            one_time_persona_data: Some(DappToWalletInteractionPersonaDataRequestItem::sample_other()),
        }
    }
}