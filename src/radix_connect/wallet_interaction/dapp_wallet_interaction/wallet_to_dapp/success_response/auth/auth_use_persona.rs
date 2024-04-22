use crate::prelude::*;

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
pub struct WalletToDappInteractionAuthUsePersonaRequestResponseItem {
    pub persona: DappWalletInteractionPersona,
}

impl HasSampleValues for WalletToDappInteractionAuthUsePersonaRequestResponseItem {
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