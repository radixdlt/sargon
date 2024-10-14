use crate::prelude::*;
use sargon::DappToWalletInteractionAuthChallengeNonce as InternalDappToWalletInteractionAuthChallengeNonce;

uniffi::custom_newtype!(DappToWalletInteractionAuthChallengeNonce, Exactly32Bytes);

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2)]
pub struct DappToWalletInteractionAuthChallengeNonce(pub Exactly32Bytes);
