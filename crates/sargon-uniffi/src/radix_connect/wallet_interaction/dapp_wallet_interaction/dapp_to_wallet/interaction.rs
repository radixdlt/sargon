use crate::prelude::*;
use sargon::DappToWalletInteraction as InternalDappToWalletInteraction;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct DappToWalletInteraction {
    pub interaction_id: WalletInteractionId,
    pub items: DappToWalletInteractionItems,
    pub metadata: DappToWalletInteractionMetadata,
}
