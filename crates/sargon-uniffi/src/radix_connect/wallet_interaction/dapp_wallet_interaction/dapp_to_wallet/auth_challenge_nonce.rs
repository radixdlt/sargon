use crate::prelude::*;
use sargon::DappToWalletInteractionAuthChallengeNonce as InternalDappToWalletInteractionAuthChallengeNonce;

#[derive(
    Debug, Clone, PartialEq, Eq, Hash,  uniffi::Record,
)]
pub struct DappToWalletInteractionAuthChallengeNonce {
    pub value: Exactly32Bytes,
}

impl From<InternalDappToWalletInteractionAuthChallengeNonce> for DappToWalletInteractionAuthChallengeNonce {
    fn from(value: InternalDappToWalletInteractionAuthChallengeNonce) -> Self {
        Self {
            value: value.0.into(),
        }
    }
}

impl Into<InternalDappToWalletInteractionAuthChallengeNonce> for DappToWalletInteractionAuthChallengeNonce {
    fn into(self) -> InternalDappToWalletInteractionAuthChallengeNonce {
        InternalDappToWalletInteractionAuthChallengeNonce(self.value.into())
    }
}
