use crate::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
pub struct DappWalletInteractionAuthLoginWithChallengeRequestResponseItem {
    pub persona: DappWalletInteractionPersona,
    pub challenge: Exactly32Bytes,
    pub proof: DappWalletInteractionAuthProof,
}
