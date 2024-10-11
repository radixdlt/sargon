use crate::prelude::*;
use sargon::DappToWalletInteractionAuthRequestItem as InternalDappToWalletInteractionAuthRequestItem;

#[derive(Clone, PartialEq, InternalConversionV2, uniffi::Enum)]
pub enum DappToWalletInteractionAuthRequestItem {
    LoginWithChallenge(
        DappToWalletInteractionAuthLoginWithChallengeRequestItem,
    ),
    LoginWithoutChallenge,
    UsePersona(DappToWalletInteractionAuthUsePersonaRequestItem),
}
