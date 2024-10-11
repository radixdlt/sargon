use crate::prelude::*;
use sargon::WalletToDappInteractionUnauthorizedRequestResponseItems as InternalWalletToDappInteractionUnauthorizedRequestResponseItems;

#[derive(Clone, PartialEq, uniffi::Record)]
pub struct WalletToDappInteractionUnauthorizedRequestResponseItems {
    pub one_time_accounts:
        Option<WalletToDappInteractionAccountsRequestResponseItem>,
    pub one_time_persona_data:
        Option<WalletToDappInteractionPersonaDataRequestResponseItem>,
}

impl From<InternalWalletToDappInteractionUnauthorizedRequestResponseItems>
    for WalletToDappInteractionUnauthorizedRequestResponseItems
{
    fn from(
        value: InternalWalletToDappInteractionUnauthorizedRequestResponseItems,
    ) -> Self {
        Self {
            one_time_accounts: value.one_time_accounts.map(Into::into),
            one_time_persona_data: value.one_time_persona_data.map(Into::into),
        }
    }
}

impl Into<InternalWalletToDappInteractionUnauthorizedRequestResponseItems>
    for WalletToDappInteractionUnauthorizedRequestResponseItems
{
    fn into(
        self,
    ) -> InternalWalletToDappInteractionUnauthorizedRequestResponseItems {
        InternalWalletToDappInteractionUnauthorizedRequestResponseItems {
            one_time_accounts: self.one_time_accounts.map(Into::into),
            one_time_persona_data: self.one_time_persona_data.map(Into::into),
        }
    }
}
