use crate::prelude::*;
use sargon::DappToWalletInteractionAuthRequestItem as InternalDappToWalletInteractionAuthRequestItem;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Enum)]
pub enum DappToWalletInteractionAuthRequestItem {
    LoginWithChallenge(
        DappToWalletInteractionAuthLoginWithChallengeRequestItem,
    ),
    LoginWithoutChallenge,
    UsePersona(DappToWalletInteractionAuthUsePersonaRequestItem),
}
