use crate::prelude::*;
use sargon::DappToWalletInteractionAuthUsePersonaRequestItem as InternalDappToWalletInteractionAuthUsePersonaRequestItem;

#[derive( Clone, PartialEq,  uniffi::Record)]
pub struct DappToWalletInteractionAuthUsePersonaRequestItem {
    pub identity_address: IdentityAddress,
}

impl From<InternalDappToWalletInteractionAuthUsePersonaRequestItem> for DappToWalletInteractionAuthUsePersonaRequestItem {
    fn from(value: InternalDappToWalletInteractionAuthUsePersonaRequestItem) -> Self {
        Self {
            identity_address: value.identity_address.into(),
        }
    }
}

impl Into<InternalDappToWalletInteractionAuthUsePersonaRequestItem> for DappToWalletInteractionAuthUsePersonaRequestItem {
    fn into(self) -> InternalDappToWalletInteractionAuthUsePersonaRequestItem {
        InternalDappToWalletInteractionAuthUsePersonaRequestItem {
            identity_address: self.identity_address.into(),
        }
    }
}

