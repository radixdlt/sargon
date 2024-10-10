use crate::prelude::*;
use sargon::DappWalletInteractionPersona as InternalDappWalletInteractionPersona;

#[derive( Clone, PartialEq,  uniffi::Record)]
pub struct DappWalletInteractionPersona {
    pub identity_address: IdentityAddress,
    pub label: String,
}

impl From<InternalDappWalletInteractionPersona> for DappWalletInteractionPersona {
    fn from(value: InternalDappWalletInteractionPersona) -> Self {
        Self {
            identity_address: value.identity_address.into(),
            label: value.label,
        }
    }
}

impl Into<InternalDappWalletInteractionPersona> for DappWalletInteractionPersona {
    fn into(self) -> InternalDappWalletInteractionPersona {
        InternalDappWalletInteractionPersona {
            identity_address: self.identity_address.into(),
            label: self.label,
        }
    }
}