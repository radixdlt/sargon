use crate::prelude::*;

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
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

impl HasSampleValues for DappWalletInteractionAuthorizedRequestResponseItems {
    fn sample() -> Self {
        Self {
            auth: DappWalletInteractionAuthRequestResponseItem::sample(),
            ongoing_accounts: Some(
                DappWalletInteractionAccountsRequestResponseItem::sample(),
            ),
            ongoing_persona_data: Some(
                DappWalletInteractionPersonaDataRequestResponseItem::sample(),
            ),
            one_time_accounts: Some(
                DappWalletInteractionAccountsRequestResponseItem::sample(),
            ),
            one_time_persona_data: Some(
                DappWalletInteractionPersonaDataRequestResponseItem::sample(),
            ),
        }
    }

    fn sample_other() -> Self {
        Self {
            auth: DappWalletInteractionAuthRequestResponseItem::sample_other(),
            ongoing_accounts: Some(
                DappWalletInteractionAccountsRequestResponseItem::sample_other(),
            ),
            ongoing_persona_data: Some(
                DappWalletInteractionPersonaDataRequestResponseItem::sample_other(),
            ),
            one_time_accounts: Some(
                DappWalletInteractionAccountsRequestResponseItem::sample_other(),
            ),
            one_time_persona_data: Some(
                DappWalletInteractionPersonaDataRequestResponseItem::sample_other(),
            ),
        }
    }
}