use crate::prelude::*;
use sargon::WalletToDappInteractionTransactionResponseItems as InternalWalletToDappInteractionTransactionResponseItems;
use sargon::WalletToDappInteractionSendTransactionResponseItem as InternalWalletToDappInteractionSendTransactionResponseItem;

#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct WalletToDappInteractionTransactionResponseItems {
    pub send: WalletToDappInteractionSendTransactionResponseItem,
}

#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct WalletToDappInteractionSendTransactionResponseItem {
    transaction_intent_hash: IntentHash,
}

impl From<InternalWalletToDappInteractionTransactionResponseItems> for WalletToDappInteractionTransactionResponseItems {
    fn from(value: InternalWalletToDappInteractionTransactionResponseItems) -> Self {
        Self {
            send: value.send.into(),
        }
    }
}

impl Into<InternalWalletToDappInteractionTransactionResponseItems> for WalletToDappInteractionTransactionResponseItems {
    fn into(self) -> InternalWalletToDappInteractionTransactionResponseItems {
        InternalWalletToDappInteractionTransactionResponseItems {
            send: self.send.into(),
        }
    }
}

impl From<InternalWalletToDappInteractionSendTransactionResponseItem> for WalletToDappInteractionSendTransactionResponseItem {
    fn from(value: InternalWalletToDappInteractionSendTransactionResponseItem) -> Self {
        Self {
            transaction_intent_hash: value.transaction_intent_hash.into(),
        }
    }
}

impl Into<InternalWalletToDappInteractionSendTransactionResponseItem> for WalletToDappInteractionSendTransactionResponseItem {
    fn into(self) -> InternalWalletToDappInteractionSendTransactionResponseItem {
        InternalWalletToDappInteractionSendTransactionResponseItem {
            transaction_intent_hash: self.transaction_intent_hash.into(),
        }
    }
}