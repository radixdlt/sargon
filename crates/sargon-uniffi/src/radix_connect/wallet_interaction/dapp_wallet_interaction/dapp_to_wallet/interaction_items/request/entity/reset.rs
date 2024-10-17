use crate::prelude::*;
use sargon::DappToWalletInteractionResetRequestItem as InternalDappToWalletInteractionResetRequestItem;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct DappToWalletInteractionResetRequestItem {
    pub accounts: bool,
    pub persona_data: bool,
}