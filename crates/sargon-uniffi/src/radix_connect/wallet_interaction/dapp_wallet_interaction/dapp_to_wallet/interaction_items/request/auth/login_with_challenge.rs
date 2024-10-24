use crate::prelude::*;
use sargon::DappToWalletInteractionAuthLoginWithChallengeRequestItem as InternalDappToWalletInteractionAuthLoginWithChallengeRequestItem;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct DappToWalletInteractionAuthLoginWithChallengeRequestItem {
    pub challenge: DappToWalletInteractionAuthChallengeNonce,
}
