use crate::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
pub struct DappWalletInteractionAccountsRequestResponseItem {
    pub accounts: Vec<WalletInteractionWalletAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge: Option<Exactly32Bytes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proofs: Option<Vec<DappWalletInteractionAccountProof>>,
}
