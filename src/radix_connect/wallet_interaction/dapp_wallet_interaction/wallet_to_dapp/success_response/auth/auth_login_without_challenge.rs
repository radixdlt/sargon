use crate::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub struct DappWalletInteractionAuthLoginWithoutChallengeRequestResponseItem {
    pub persona: DappWalletInteractionPersona,
}
