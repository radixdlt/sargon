use crate::prelude::*;
use sargon::WalletToDappInteractionSuccessResponse as InternalWalletToDappInteractionSuccessResponse;

#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct WalletToDappInteractionSuccessResponse {
    pub interaction_id: WalletInteractionId,
    pub items: WalletToDappInteractionResponseItems,
}

impl From<InternalWalletToDappInteractionSuccessResponse> for WalletToDappInteractionSuccessResponse {
    fn from(value: InternalWalletToDappInteractionSuccessResponse) -> Self {
        Self {
            interaction_id: value.interaction_id.into(),
            items: value.items.into(),
        }
    }
}

impl Into<InternalWalletToDappInteractionSuccessResponse> for WalletToDappInteractionSuccessResponse {
    fn into(self) -> InternalWalletToDappInteractionSuccessResponse {
        InternalWalletToDappInteractionSuccessResponse {
            interaction_id: self.interaction_id.into(),
            items: self.items.into(),
        }
    }
}