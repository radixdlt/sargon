use crate::prelude::*;
use sargon::DappToWalletInteractionAuthorizedRequestItems as InternalDappToWalletInteractionAuthorizedRequestItems;

#[derive(Debug, Clone, PartialEq, uniffi::Record)]
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

impl From<InternalDappToWalletInteractionAuthorizedRequestItems> for DappToWalletInteractionAuthorizedRequestItems {
    fn from(value: InternalDappToWalletInteractionAuthorizedRequestItems) -> Self {
        Self {
            auth: value.auth.into(),
            reset: value.reset.map(Into::into),
            ongoing_accounts: value.ongoing_accounts.map(Into::into),
            ongoing_persona_data: value.ongoing_persona_data.map(Into::into),
            one_time_accounts: value.one_time_accounts.map(Into::into),
            one_time_persona_data: value.one_time_persona_data.map(Into::into),
        }
    }
}

impl Into<InternalDappToWalletInteractionAuthorizedRequestItems> for DappToWalletInteractionAuthorizedRequestItems {
    fn into(self) -> InternalDappToWalletInteractionAuthorizedRequestItems {
        InternalDappToWalletInteractionAuthorizedRequestItems {
            auth: self.auth.into(),
            reset: self.reset.map(Into::into),
            ongoing_accounts: self.ongoing_accounts.map(Into::into),
            ongoing_persona_data: self.ongoing_persona_data.map(Into::into),
            one_time_accounts: self.one_time_accounts.map(Into::into),
            one_time_persona_data: self.one_time_persona_data.map(Into::into),
        }
    }
}