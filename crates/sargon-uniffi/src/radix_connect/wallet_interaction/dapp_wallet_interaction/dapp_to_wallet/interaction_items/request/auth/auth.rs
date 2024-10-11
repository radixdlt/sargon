use crate::prelude::*;
use sargon::DappToWalletInteractionAuthRequestItem as InternalDappToWalletInteractionAuthRequestItem;

#[derive(Clone, PartialEq, uniffi::Enum)]
pub enum DappToWalletInteractionAuthRequestItem {
    LoginWithChallenge(
        DappToWalletInteractionAuthLoginWithChallengeRequestItem,
    ),
    LoginWithoutChallenge,
    UsePersona(DappToWalletInteractionAuthUsePersonaRequestItem),
}

impl From<InternalDappToWalletInteractionAuthRequestItem>
    for DappToWalletInteractionAuthRequestItem
{
    fn from(value: InternalDappToWalletInteractionAuthRequestItem) -> Self {
        match value {
            InternalDappToWalletInteractionAuthRequestItem::LoginWithChallenge(value) => {
                DappToWalletInteractionAuthRequestItem::LoginWithChallenge(value.into())
            }
            InternalDappToWalletInteractionAuthRequestItem::LoginWithoutChallenge => {
                DappToWalletInteractionAuthRequestItem::LoginWithoutChallenge
            }
            InternalDappToWalletInteractionAuthRequestItem::UsePersona(value) => {
                DappToWalletInteractionAuthRequestItem::UsePersona(value.into())
            }
        }
    }
}

impl Into<InternalDappToWalletInteractionAuthRequestItem>
    for DappToWalletInteractionAuthRequestItem
{
    fn into(self) -> InternalDappToWalletInteractionAuthRequestItem {
        match self {
            DappToWalletInteractionAuthRequestItem::LoginWithChallenge(value) => {
                InternalDappToWalletInteractionAuthRequestItem::LoginWithChallenge(value.into())
            }
            DappToWalletInteractionAuthRequestItem::LoginWithoutChallenge => {
                InternalDappToWalletInteractionAuthRequestItem::LoginWithoutChallenge
            }
            DappToWalletInteractionAuthRequestItem::UsePersona(value) => {
                InternalDappToWalletInteractionAuthRequestItem::UsePersona(value.into())
            }
        }
    }
}
