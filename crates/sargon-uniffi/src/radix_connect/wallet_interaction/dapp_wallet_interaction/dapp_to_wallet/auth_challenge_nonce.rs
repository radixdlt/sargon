use crate::prelude::*;
use sargon::DappToWalletInteractionAuthChallengeNonce as InternalDappToWalletInteractionAuthChallengeNonce;

uniffi::custom_newtype!(
    DappToWalletInteractionAuthChallengeNonce,
    Exactly32Bytes
);

#[derive(
    Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash,
)]
pub struct DappToWalletInteractionAuthChallengeNonce(pub Exactly32Bytes);

impl From<InternalDappToWalletInteractionAuthChallengeNonce> for DappToWalletInteractionAuthChallengeNonce {
    fn from(value: InternalDappToWalletInteractionAuthChallengeNonce) -> Self {
        Self(value.0.into())
    }
}

impl Into<InternalDappToWalletInteractionAuthChallengeNonce> for DappToWalletInteractionAuthChallengeNonce {
    fn into(self) -> InternalDappToWalletInteractionAuthChallengeNonce {
        InternalDappToWalletInteractionAuthChallengeNonce(self.0.into())
    }
}
