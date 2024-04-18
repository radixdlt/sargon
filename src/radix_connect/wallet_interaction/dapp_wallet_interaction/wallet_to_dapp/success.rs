use crate::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DappWalletInteractionSuccessResponse {
    pub interaction_id: WalletInteractionId,
    pub items: DappWalletInteractionResponseItems,
}
