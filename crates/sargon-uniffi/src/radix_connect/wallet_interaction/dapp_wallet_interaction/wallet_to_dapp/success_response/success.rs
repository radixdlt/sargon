use crate::prelude::*;
use sargon::WalletToDappInteractionSuccessResponse as InternalWalletToDappInteractionSuccessResponse;

#[derive(Clone, PartialEq, InternalConversionV2, uniffi::Record)]
pub struct WalletToDappInteractionSuccessResponse {
    pub interaction_id: WalletInteractionId,
    pub items: WalletToDappInteractionResponseItems,
}
