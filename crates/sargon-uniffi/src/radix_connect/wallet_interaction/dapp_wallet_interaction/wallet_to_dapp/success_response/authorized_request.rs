use crate::prelude::*;
use sargon::WalletToDappInteractionAuthorizedRequestResponseItems as InternalWalletToDappInteractionAuthorizedRequestResponseItems;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
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
