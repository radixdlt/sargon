use crate::prelude::*;

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
pub struct WalletToDappInteractionAuthLoginWithChallengeRequestResponseItem {
    pub persona: DappWalletInteractionPersona,
    pub challenge: Exactly32Bytes,
    pub proof: WalletToDappInteractionAuthProof,
}

impl HasSampleValues
    for WalletToDappInteractionAuthLoginWithChallengeRequestResponseItem
{
    fn sample() -> Self {
        Self {
            persona: DappWalletInteractionPersona::sample(),
            challenge: Exactly32Bytes::sample(),
            proof: WalletToDappInteractionAuthProof::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            persona: DappWalletInteractionPersona::sample_other(),
            challenge: Exactly32Bytes::sample_other(),
            proof: WalletToDappInteractionAuthProof::sample_other(),
        }
    }
}
