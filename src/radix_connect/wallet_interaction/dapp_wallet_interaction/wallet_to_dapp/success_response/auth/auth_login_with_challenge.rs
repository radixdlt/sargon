use crate::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub struct DappWalletInteractionAuthLoginWithChallengeRequestResponseItem {
    pub persona: DappWalletInteractionPersona,
    pub challenge: Exactly32Bytes,
    pub proof: DappWalletInteractionAuthProof,
}
