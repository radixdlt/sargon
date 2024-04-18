use crate::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
pub struct DappWalletInteractionAuthUsePersonaRequestResponseItem {
    pub persona: DappWalletInteractionPersona,
}
