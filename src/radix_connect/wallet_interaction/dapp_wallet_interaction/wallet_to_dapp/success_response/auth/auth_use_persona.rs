use crate::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub struct DappWalletInteractionAuthUsePersonaRequestResponseItem {
    pub persona: DappWalletInteractionPersona,
}
