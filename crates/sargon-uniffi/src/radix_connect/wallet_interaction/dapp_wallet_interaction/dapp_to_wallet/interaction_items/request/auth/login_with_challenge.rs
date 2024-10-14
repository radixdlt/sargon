use crate::prelude::*;
use sargon::DappToWalletInteractionAuthLoginWithChallengeRequestItem as InternalDappToWalletInteractionAuthLoginWithChallengeRequestItem;

#[derive(Clone, PartialEq, InternalConversionV2, uniffi::Record)]
pub struct DappToWalletInteractionAuthLoginWithChallengeRequestItem {
    pub challenge: DappToWalletInteractionAuthChallengeNonce,
}
