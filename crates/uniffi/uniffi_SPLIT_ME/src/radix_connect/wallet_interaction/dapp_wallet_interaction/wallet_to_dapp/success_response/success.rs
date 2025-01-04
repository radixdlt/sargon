use crate::prelude::*;
use sargon::WalletToDappInteractionSuccessResponse as InternalWalletToDappInteractionSuccessResponse;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct WalletToDappInteractionSuccessResponse {
    pub interaction_id: WalletInteractionId,
    pub items: WalletToDappInteractionResponseItems,
}
