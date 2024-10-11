use crate::prelude::*;
use sargon::WalletToDappInteractionSendTransactionResponseItem as InternalWalletToDappInteractionSendTransactionResponseItem;
use sargon::WalletToDappInteractionTransactionResponseItems as InternalWalletToDappInteractionTransactionResponseItems;

#[derive(Clone, PartialEq, uniffi::Record)]
pub struct WalletToDappInteractionTransactionResponseItems {
    pub send: WalletToDappInteractionSendTransactionResponseItem,
}

#[derive(Clone, PartialEq, uniffi::Record)]
pub struct WalletToDappInteractionSendTransactionResponseItem {
    transaction_intent_hash: IntentHash,
}

impl From<InternalWalletToDappInteractionTransactionResponseItems>
    for WalletToDappInteractionTransactionResponseItems
{
    fn from(
        value: InternalWalletToDappInteractionTransactionResponseItems,
    ) -> Self {
        Self {
            send: value.send.into(),
        }
    }
}

impl Into<InternalWalletToDappInteractionTransactionResponseItems>
    for WalletToDappInteractionTransactionResponseItems
{
    fn into(self) -> InternalWalletToDappInteractionTransactionResponseItems {
        InternalWalletToDappInteractionTransactionResponseItems {
            send: self.send.into(),
        }
    }
}

impl From<InternalWalletToDappInteractionSendTransactionResponseItem>
    for WalletToDappInteractionSendTransactionResponseItem
{
    fn from(
        value: InternalWalletToDappInteractionSendTransactionResponseItem,
    ) -> Self {
        Self {
            transaction_intent_hash: value.transaction_intent_hash.into(),
        }
    }
}

impl Into<InternalWalletToDappInteractionSendTransactionResponseItem>
    for WalletToDappInteractionSendTransactionResponseItem
{
    fn into(
        self,
    ) -> InternalWalletToDappInteractionSendTransactionResponseItem {
        InternalWalletToDappInteractionSendTransactionResponseItem {
            transaction_intent_hash: self.transaction_intent_hash.into(),
        }
    }
}
