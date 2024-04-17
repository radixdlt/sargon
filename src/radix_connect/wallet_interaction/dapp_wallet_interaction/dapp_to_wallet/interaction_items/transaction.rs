use crate::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, PartialEq, uniffi::Record)]
pub struct DappToWalletInteractionTransactionItems {
    pub send: DappToWalletInteractionSendTransactionItem,
}

#[derive(Debug, Deserialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionSendTransactionItem {
    pub transaction_manifest: String,
    pub version: TXVersion,
    pub blobs: Option<Vec<String>>,
    pub message: Option<String>,
}

uniffi::custom_newtype!(TXVersion, u64);
#[derive(Debug, Deserialize, PartialEq)]
pub struct TXVersion(u64);

impl From<u64> for TXVersion {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl HasSampleValues for TXVersion {
    fn sample() -> Self {
        Self(1)
    }

    fn sample_other() -> Self {
        Self(2)
    }
}

impl HasSampleValues for DappToWalletInteractionTransactionItems {
    fn sample() -> Self {
        Self {
            send: DappToWalletInteractionSendTransactionItem::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            send: DappToWalletInteractionSendTransactionItem::sample_other(),
        }
    }
}

impl HasSampleValues for DappToWalletInteractionSendTransactionItem {
    fn sample() -> Self {
        Self {
            transaction_manifest: "transaction_manifest".to_string(),
            version: TXVersion::sample(),
            blobs: Some(vec!["blob".to_string()]),
            message: Some("message".to_string()),
        }
    }

    fn sample_other() -> Self {
        Self {
            transaction_manifest: "transaction_manifest_other".to_string(),
            version: TXVersion::sample_other(),
            blobs: Some(vec!["blob_other".to_string()]),
            message: Some("message_other".to_string()),
        }
    }
}