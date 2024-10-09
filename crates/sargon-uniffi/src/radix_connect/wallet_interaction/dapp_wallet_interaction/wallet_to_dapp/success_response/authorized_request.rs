use crate::prelude::*;
use sargon::WalletToDappInteractionAuthorizedRequestResponseItems as InternalWalletToDappInteractionAuthorizedRequestResponseItems;

#[derive(Debug, Clone, PartialEq,  uniffi::Record)]
pub struct WalletToDappInteractionAuthorizedRequestResponseItems {
    pub auth: WalletToDappInteractionAuthRequestResponseItem,
    pub ongoing_accounts:
        Option<WalletToDappInteractionAccountsRequestResponseItem>,
    pub ongoing_persona_data:
        Option<WalletToDappInteractionPersonaDataRequestResponseItem>,
    pub one_time_accounts:
        Option<WalletToDappInteractionAccountsRequestResponseItem>,
    pub one_time_persona_data:
        Option<WalletToDappInteractionPersonaDataRequestResponseItem>,
}

impl From<InternalWalletToDappInteractionAuthorizedRequestResponseItems>
    for WalletToDappInteractionAuthorizedRequestResponseItems
{
    fn from(
        value: InternalWalletToDappInteractionAuthorizedRequestResponseItems,
    ) -> Self {
        Self {
            auth: value.auth.into(),
            ongoing_accounts: value.ongoing_accounts.map(Into::into),
            ongoing_persona_data: value.ongoing_persona_data.map(Into::into),
            one_time_accounts: value.one_time_accounts.map(Into::into),
            one_time_persona_data: value.one_time_persona_data.map(Into::into),
        }
    }
}

impl Into<InternalWalletToDappInteractionAuthorizedRequestResponseItems>
    for WalletToDappInteractionAuthorizedRequestResponseItems
{
    fn into(self) -> InternalWalletToDappInteractionAuthorizedRequestResponseItems {
        InternalWalletToDappInteractionAuthorizedRequestResponseItems {
            auth: self.auth.into(),
            ongoing_accounts: self.ongoing_accounts.map(Into::into),
            ongoing_persona_data: self.ongoing_persona_data.map(Into::into),
            one_time_accounts: self.one_time_accounts.map(Into::into),
            one_time_persona_data: self.one_time_persona_data.map(Into::into),
        }
    }
}