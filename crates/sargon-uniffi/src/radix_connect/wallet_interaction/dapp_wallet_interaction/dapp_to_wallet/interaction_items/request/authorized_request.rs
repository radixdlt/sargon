use crate::prelude::*;
use sargon::DappToWalletInteractionAuthorizedRequestItems as InternalDappToWalletInteractionAuthorizedRequestItems;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
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
