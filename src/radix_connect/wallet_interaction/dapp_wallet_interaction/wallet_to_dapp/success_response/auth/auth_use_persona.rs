use crate::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
pub struct DappWalletInteractionAuthUsePersonaRequestResponseItem {
    pub persona: DappWalletInteractionPersona,
}

impl HasSampleValues for DappWalletInteractionAuthUsePersonaRequestResponseItem {
    fn sample() -> Self {
        Self {
            persona: DappWalletInteractionPersona::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            persona: DappWalletInteractionPersona::sample_other(),
        }
    }
}