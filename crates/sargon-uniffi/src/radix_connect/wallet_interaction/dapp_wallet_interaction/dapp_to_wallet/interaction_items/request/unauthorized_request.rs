use crate::prelude::*;
use sargon::DappToWalletInteractionUnauthorizedRequestItems as InternalDappToWalletInteractionUnauthorizedRequestItems;

#[derive(Debug, Clone, PartialEq,  uniffi::Record)]
pub struct DappToWalletInteractionUnauthorizedRequestItems {
    pub one_time_accounts: Option<DappToWalletInteractionAccountsRequestItem>,
    pub one_time_persona_data:
        Option<DappToWalletInteractionPersonaDataRequestItem>,
}

impl From<InternalDappToWalletInteractionUnauthorizedRequestItems> for DappToWalletInteractionUnauthorizedRequestItems {
    fn from(value: InternalDappToWalletInteractionUnauthorizedRequestItems) -> Self {
        Self {
            one_time_accounts: value.one_time_accounts.map(Into::into),
            one_time_persona_data: value.one_time_persona_data.map(Into::into),
        }
    }
}

impl Into<InternalDappToWalletInteractionUnauthorizedRequestItems> for DappToWalletInteractionUnauthorizedRequestItems {
    fn into(self) -> InternalDappToWalletInteractionUnauthorizedRequestItems {
        InternalDappToWalletInteractionUnauthorizedRequestItems {
            one_time_accounts: self.one_time_accounts.map(Into::into),
            one_time_persona_data: self.one_time_persona_data.map(Into::into),
        }
    }
}