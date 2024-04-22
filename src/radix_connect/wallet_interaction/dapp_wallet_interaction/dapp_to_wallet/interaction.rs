use super::interaction_items::DappToWalletInteractionItems;
use super::interaction_metadata::DappToWalletInteractionMetadata;
use crate::prelude::*;

#[derive(Debug, Deserialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteraction {
    pub interaction_id: WalletInteractionId,
    pub items: DappToWalletInteractionItems,
    pub metadata: DappToWalletInteractionMetadata,
}

#[derive(Debug, Deserialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionUnvalidated {
    pub interaction_id: WalletInteractionId,
    pub items: DappToWalletInteractionItems,
    pub metadata: DappToWalletInteractionMetadataUnvalidated,
}
