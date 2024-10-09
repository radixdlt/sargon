use crate::prelude::*;
use sargon::DappToWalletInteractionUnvalidated as InternalDappToWalletInteractionUnvalidated;

json_data_convertible!(DappToWalletInteractionUnvalidated);

#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct DappToWalletInteractionUnvalidated {
    pub interaction_id: WalletInteractionId,
    pub items: DappToWalletInteractionItems,
    pub metadata: DappToWalletInteractionMetadataUnvalidated,
}

impl From<InternalDappToWalletInteractionUnvalidated> for DappToWalletInteractionUnvalidated {
    fn from(value: InternalDappToWalletInteractionUnvalidated) -> Self {
        Self {
            interaction_id: value.interaction_id.into(),
            items: value.items.into(),
            metadata: value.metadata.into(),
        }
    }
}

impl Into<InternalDappToWalletInteractionUnvalidated> for DappToWalletInteractionUnvalidated {
    fn into(self) -> InternalDappToWalletInteractionUnvalidated {
        InternalDappToWalletInteractionUnvalidated {
            interaction_id: self.interaction_id.into(),
            items: self.items.into(),
            metadata: self.metadata.into(),
        }
    }
}