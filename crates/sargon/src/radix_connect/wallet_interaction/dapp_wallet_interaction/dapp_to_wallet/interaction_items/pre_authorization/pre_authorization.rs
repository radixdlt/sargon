use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, uniffi::Record)]
pub struct DappToWalletInteractionPreAuthorizationItems {
    pub subintent: Option<DappToWalletInteractionSubintentRequestItem>,
}
