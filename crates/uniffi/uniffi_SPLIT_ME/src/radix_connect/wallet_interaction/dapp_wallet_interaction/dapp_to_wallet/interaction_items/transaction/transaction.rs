use crate::prelude::*;
use sargon::DappToWalletInteractionSendTransactionItem as InternalDappToWalletInteractionSendTransactionItem;
use sargon::DappToWalletInteractionTransactionItems as InternalDappToWalletInteractionTransactionItems;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct DappToWalletInteractionTransactionItems {
    pub send: DappToWalletInteractionSendTransactionItem,
}

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct DappToWalletInteractionSendTransactionItem {
    pub unvalidated_manifest: UnvalidatedTransactionManifest,
    pub version: TXVersion,
    pub message: Option<String>,
}
