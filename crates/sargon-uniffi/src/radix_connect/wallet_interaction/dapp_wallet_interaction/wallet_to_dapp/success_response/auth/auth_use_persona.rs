use crate::prelude::*;
use sargon::WalletToDappInteractionAuthUsePersonaRequestResponseItem as InternalWalletToDappInteractionAuthUsePersonaRequestResponseItem;

#[derive(Clone, PartialEq, InternalConversionV2, uniffi::Record)]
pub struct WalletToDappInteractionAuthUsePersonaRequestResponseItem {
    pub persona: DappWalletInteractionPersona,
}