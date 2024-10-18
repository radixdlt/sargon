use crate::prelude::*;
use sargon::DappToWalletInteractionAccountsRequestItem as InternalDappToWalletInteractionAccountsRequestItem;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct DappToWalletInteractionAccountsRequestItem {
    pub number_of_accounts: RequestedQuantity,
    pub challenge: Option<DappToWalletInteractionAuthChallengeNonce>,
}
