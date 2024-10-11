use crate::prelude::*;
use sargon::DappToWalletInteraction as InternalDappToWalletInteraction;

#[derive( Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct DappToWalletInteraction {
    pub interaction_id: WalletInteractionId,
    pub items: DappToWalletInteractionItems,
    pub metadata: DappToWalletInteractionMetadata,
}

impl From<InternalDappToWalletInteraction> for DappToWalletInteraction {
    fn from(value: InternalDappToWalletInteraction) -> Self {
        Self {
            interaction_id: value.interaction_id.into(),
            items: value.items.into(),
            metadata: value.metadata.into(),
        }
    }
}

impl Into<InternalDappToWalletInteraction> for DappToWalletInteraction {
    fn into(self) -> InternalDappToWalletInteraction {
        InternalDappToWalletInteraction {
            interaction_id: self.interaction_id.into(),
            items: self.items.into(),
            metadata: self.metadata.into(),
        }
    }
}
