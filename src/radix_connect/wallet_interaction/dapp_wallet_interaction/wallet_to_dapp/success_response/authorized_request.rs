use crate::prelude::*;

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct WalletToDappInteractionAuthorizedRequestResponseItems {
    pub auth: WalletToDappInteractionAuthRequestResponseItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ongoing_accounts:
        Option<WalletToDappInteractionAccountsRequestResponseItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ongoing_persona_data:
        Option<WalletToDappInteractionPersonaDataRequestResponseItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_accounts:
        Option<WalletToDappInteractionAccountsRequestResponseItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_persona_data:
        Option<WalletToDappInteractionPersonaDataRequestResponseItem>,
}

impl HasSampleValues for WalletToDappInteractionAuthorizedRequestResponseItems {
    fn sample() -> Self {
        Self {
            auth: WalletToDappInteractionAuthRequestResponseItem::sample(),
            ongoing_accounts: Some(
                WalletToDappInteractionAccountsRequestResponseItem::sample(),
            ),
            ongoing_persona_data: Some(
                WalletToDappInteractionPersonaDataRequestResponseItem::sample(),
            ),
            one_time_accounts: Some(
                WalletToDappInteractionAccountsRequestResponseItem::sample(),
            ),
            one_time_persona_data: Some(
                WalletToDappInteractionPersonaDataRequestResponseItem::sample(),
            ),
        }
    }

    fn sample_other() -> Self {
        Self {
            auth: WalletToDappInteractionAuthRequestResponseItem::sample_other(),
            ongoing_accounts: Some(
                WalletToDappInteractionAccountsRequestResponseItem::sample_other(),
            ),
            ongoing_persona_data: Some(
                WalletToDappInteractionPersonaDataRequestResponseItem::sample_other(),
            ),
            one_time_accounts: Some(
                WalletToDappInteractionAccountsRequestResponseItem::sample_other(),
            ),
            one_time_persona_data: Some(
                WalletToDappInteractionPersonaDataRequestResponseItem::sample_other(),
            ),
        }
    }
}