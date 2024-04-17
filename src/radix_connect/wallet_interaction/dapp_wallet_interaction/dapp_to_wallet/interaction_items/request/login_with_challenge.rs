use crate::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, uniffi::Record)]
pub struct DappToWalletInteractionAuthLoginWithChallengeRequestItem {
    pub challenge: Exactly32Bytes,
}

impl HasSampleValues for DappToWalletInteractionAuthLoginWithChallengeRequestItem {
    fn sample() -> Self {
        Self {
            challenge: Exactly32Bytes::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            challenge: Exactly32Bytes::sample_other(),
        }
    }
}