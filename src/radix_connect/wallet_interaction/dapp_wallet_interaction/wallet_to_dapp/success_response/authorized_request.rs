use crate::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DappWalletInteractionAuthorizedRequestResponseItems {
    pub auth: DappWalletInteractionAuthRequestResponseItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ongoing_accounts:
        Option<DappWalletInteractionAccountsRequestResponseItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ongoing_persona_data:
        Option<DappWalletInteractionPersonaDataRequestResponseItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_accounts:
        Option<DappWalletInteractionAccountsRequestResponseItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_persona_data:
        Option<DappWalletInteractionPersonaDataRequestResponseItem>,
}
