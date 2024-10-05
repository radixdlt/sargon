use crate::prelude::*;
use sargon::DappToWalletInteractionResetRequestItem as InternalDappToWalletInteractionResetRequestItem;

#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct DappToWalletInteractionResetRequestItem {
    pub accounts: bool,
    pub persona_data: bool,
}

impl From<InternalDappToWalletInteractionResetRequestItem> for DappToWalletInteractionResetRequestItem {
    fn from(value: InternalDappToWalletInteractionResetRequestItem) -> Self {
        Self {
            accounts: value.accounts,
            persona_data: value.persona_data,
        }
    }
}

impl Into<InternalDappToWalletInteractionResetRequestItem> for DappToWalletInteractionResetRequestItem {
    fn into(self) -> InternalDappToWalletInteractionResetRequestItem {
        InternalDappToWalletInteractionResetRequestItem {
            accounts: self.accounts,
            persona_data: self.persona_data,
        }
    }
}