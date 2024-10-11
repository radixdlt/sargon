use crate::prelude::*;
use sargon::WalletToDappInteractionAuthRequestResponseItem as InternalWalletToDappInteractionAuthRequestResponseItem;

#[derive(Clone, PartialEq, uniffi::Enum)]
pub enum WalletToDappInteractionAuthRequestResponseItem {
    UsePersona(WalletToDappInteractionAuthUsePersonaRequestResponseItem),
    LoginWithoutChallenge(
        WalletToDappInteractionAuthLoginWithoutChallengeRequestResponseItem,
    ),
    LoginWithChallenge(
        WalletToDappInteractionAuthLoginWithChallengeRequestResponseItem,
    ),
}

impl From<InternalWalletToDappInteractionAuthRequestResponseItem>
    for WalletToDappInteractionAuthRequestResponseItem
{
    fn from(
        value: InternalWalletToDappInteractionAuthRequestResponseItem,
    ) -> Self {
        match value {
            InternalWalletToDappInteractionAuthRequestResponseItem::UsePersona(value) => WalletToDappInteractionAuthRequestResponseItem::UsePersona(value.into()),
            InternalWalletToDappInteractionAuthRequestResponseItem::LoginWithoutChallenge(value) => WalletToDappInteractionAuthRequestResponseItem::LoginWithoutChallenge(value.into()),
            InternalWalletToDappInteractionAuthRequestResponseItem::LoginWithChallenge(value) => WalletToDappInteractionAuthRequestResponseItem::LoginWithChallenge(value.into()),
        }
    }
}

impl Into<InternalWalletToDappInteractionAuthRequestResponseItem>
    for WalletToDappInteractionAuthRequestResponseItem
{
    fn into(self) -> InternalWalletToDappInteractionAuthRequestResponseItem {
        match self {
            WalletToDappInteractionAuthRequestResponseItem::UsePersona(value) => InternalWalletToDappInteractionAuthRequestResponseItem::UsePersona(value.into()),
            WalletToDappInteractionAuthRequestResponseItem::LoginWithoutChallenge(value) => InternalWalletToDappInteractionAuthRequestResponseItem::LoginWithoutChallenge(value.into()),
            WalletToDappInteractionAuthRequestResponseItem::LoginWithChallenge(value) => InternalWalletToDappInteractionAuthRequestResponseItem::LoginWithChallenge(value.into()),
        }
    }
}
