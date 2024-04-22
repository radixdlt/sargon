use crate::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
pub struct DappWalletInteractionAuthLoginWithChallengeRequestResponseItem {
    pub persona: DappWalletInteractionPersona,
    pub challenge: Exactly32Bytes,
    pub proof: DappWalletInteractionAuthProof,
}

impl HasSampleValues for DappWalletInteractionAuthLoginWithChallengeRequestResponseItem {
    fn sample() -> Self {
        Self {
            persona: DappWalletInteractionPersona::sample(),
            challenge: Exactly32Bytes::sample(),
            proof: DappWalletInteractionAuthProof::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            persona: DappWalletInteractionPersona::sample_other(),
            challenge: Exactly32Bytes::sample_other(),
            proof: DappWalletInteractionAuthProof::sample_other(),
        }
    }
}
