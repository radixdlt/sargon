use crate::prelude::*;
use sargon::DappToWalletInteractionResetRequestItem as InternalDappToWalletInteractionResetRequestItem;

#[derive(Clone, PartialEq, InternalConversionV2, uniffi::Record)]
pub struct DappToWalletInteractionResetRequestItem {
    pub accounts: bool,
    pub persona_data: bool,
}