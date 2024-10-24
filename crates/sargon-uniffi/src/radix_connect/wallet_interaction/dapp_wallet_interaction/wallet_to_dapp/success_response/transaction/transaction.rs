use crate::prelude::*;
use sargon::WalletToDappInteractionSendTransactionResponseItem as InternalWalletToDappInteractionSendTransactionResponseItem;
use sargon::WalletToDappInteractionTransactionResponseItems as InternalWalletToDappInteractionTransactionResponseItems;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct WalletToDappInteractionTransactionResponseItems {
    pub send: WalletToDappInteractionSendTransactionResponseItem,
}

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct WalletToDappInteractionSendTransactionResponseItem {
    pub transaction_intent_hash: TransactionIntentHash,
}
