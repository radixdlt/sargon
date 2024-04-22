use crate::prelude::*;

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct WalletToDappInteractionUnauthorizedRequestResponseItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_accounts:
        Option<WalletToDappInteractionAccountsRequestResponseItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_persona_data:
        Option<WalletToDappInteractionPersonaDataRequestResponseItem>,
}

impl HasSampleValues
    for WalletToDappInteractionUnauthorizedRequestResponseItems
{
    fn sample() -> Self {
        Self {
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
            one_time_accounts: Some(WalletToDappInteractionAccountsRequestResponseItem::sample_other()),
            one_time_persona_data: Some(WalletToDappInteractionPersonaDataRequestResponseItem::sample_other()),
        }
    }
}
