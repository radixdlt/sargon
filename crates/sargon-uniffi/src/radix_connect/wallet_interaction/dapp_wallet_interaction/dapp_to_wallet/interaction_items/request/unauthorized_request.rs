use crate::prelude::*;
use sargon::DappToWalletInteractionUnauthorizedRequestItems as InternalDappToWalletInteractionUnauthorizedRequestItems;

#[derive(Clone, PartialEq, InternalConversionV2, uniffi::Record)]
pub struct DappToWalletInteractionUnauthorizedRequestItems {
    pub one_time_accounts: Option<DappToWalletInteractionAccountsRequestItem>,
    pub one_time_persona_data:
        Option<DappToWalletInteractionPersonaDataRequestItem>,
}