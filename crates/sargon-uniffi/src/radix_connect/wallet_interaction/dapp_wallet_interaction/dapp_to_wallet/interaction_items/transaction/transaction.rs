use crate::prelude::*;
use sargon::DappToWalletInteractionTransactionItems as InternalDappToWalletInteractionTransactionItems;
use sargon::DappToWalletInteractionSendTransactionItem as InternalDappToWalletInteractionSendTransactionItem;

#[derive( Clone, PartialEq,  uniffi::Record)]
pub struct DappToWalletInteractionTransactionItems {
    pub send: DappToWalletInteractionSendTransactionItem,
}

#[derive( Clone, PartialEq,  uniffi::Record)]
pub struct DappToWalletInteractionSendTransactionItem {
    pub unvalidated_manifest: UnvalidatedTransactionManifest,
    pub version: TXVersion,
    pub message: Option<String>,
}

impl From<InternalDappToWalletInteractionTransactionItems> for DappToWalletInteractionTransactionItems {
    fn from(value: InternalDappToWalletInteractionTransactionItems) -> Self {
        Self {
            send: value.send.into(),
        }
    }
}

impl Into<InternalDappToWalletInteractionTransactionItems> for DappToWalletInteractionTransactionItems {
    fn into(self) -> InternalDappToWalletInteractionTransactionItems {
        InternalDappToWalletInteractionTransactionItems {
            send: self.send.into(),
        }
    }
}

impl From<InternalDappToWalletInteractionSendTransactionItem> for DappToWalletInteractionSendTransactionItem {
    fn from(value: InternalDappToWalletInteractionSendTransactionItem) -> Self {
        Self {
            unvalidated_manifest: value.unvalidated_manifest.into(),
            version: value.version.into(),
            message: value.message,
        }
    }
}

impl Into<InternalDappToWalletInteractionSendTransactionItem> for DappToWalletInteractionSendTransactionItem {
    fn into(self) -> InternalDappToWalletInteractionSendTransactionItem {
        InternalDappToWalletInteractionSendTransactionItem {
            unvalidated_manifest: self.unvalidated_manifest.into(),
            version: self.version.into(),
            message: self.message,
        }
    }
}