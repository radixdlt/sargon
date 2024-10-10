use crate::prelude::*;
use sargon::WalletToDappInteractionAuthUsePersonaRequestResponseItem as InternalWalletToDappInteractionAuthUsePersonaRequestResponseItem;

#[derive( Clone, PartialEq,  uniffi::Record)]
pub struct WalletToDappInteractionAuthUsePersonaRequestResponseItem {
    pub persona: DappWalletInteractionPersona,
}

impl From<InternalWalletToDappInteractionAuthUsePersonaRequestResponseItem> for WalletToDappInteractionAuthUsePersonaRequestResponseItem {
    fn from(value: InternalWalletToDappInteractionAuthUsePersonaRequestResponseItem) -> Self {
        Self {
            persona: value.persona.into(),
        }
    }
}

impl Into<InternalWalletToDappInteractionAuthUsePersonaRequestResponseItem> for WalletToDappInteractionAuthUsePersonaRequestResponseItem {
    fn into(self) -> InternalWalletToDappInteractionAuthUsePersonaRequestResponseItem {
        InternalWalletToDappInteractionAuthUsePersonaRequestResponseItem {
            persona: self.persona.into(),
        }
    }
}