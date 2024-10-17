use crate::prelude::*;
use sargon::WalletToDappInteractionAuthUsePersonaRequestResponseItem as InternalWalletToDappInteractionAuthUsePersonaRequestResponseItem;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct WalletToDappInteractionAuthUsePersonaRequestResponseItem {
    pub persona: DappWalletInteractionPersona,
}