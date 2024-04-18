use crate::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct WalletInteractionWalletAccount {
    pub address: AccountAddress,
    pub label: String,
    pub appearance_id: AppearanceID,
}
