use crate::prelude::*;
use sargon::WalletToDappInteractionUnauthorizedRequestResponseItems as InternalWalletToDappInteractionUnauthorizedRequestResponseItems;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct WalletToDappInteractionUnauthorizedRequestResponseItems {
    pub one_time_accounts:
        Option<WalletToDappInteractionAccountsRequestResponseItem>,
    pub one_time_persona_data:
        Option<WalletToDappInteractionPersonaDataRequestResponseItem>,
}
