use crate::prelude::*;
use sargon::WalletToDappInteractionAccountsRequestResponseItem as InternalWalletToDappInteractionAccountsRequestResponseItem;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct WalletToDappInteractionAccountsRequestResponseItem {
    pub accounts: Vec<WalletInteractionWalletAccount>,
    pub challenge: Option<DappToWalletInteractionAuthChallengeNonce>,
    pub proofs: Option<Vec<WalletToDappInteractionAccountProof>>,
}
