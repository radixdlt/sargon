use crate::prelude::*;

#[derive(Debug, Deserialize, Serialize, PartialEq, uniffi::Record)]
pub struct DappToWalletInteractionTransactionItems {
    pub send: DappToWalletInteractionSendTransactionItem,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionSendTransactionItem {
    pub transaction_manifest: String,
    pub version: TXVersion,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blobs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl DappToWalletInteractionSendTransactionItem {
    pub fn new(
        transaction_manifest: impl AsRef<str>,
        version: impl Into<TXVersion>,
        blobs: impl Into<Option<Vec<String>>>,
        message: impl Into<Option<String>>,
    ) -> Self {
        Self {
            transaction_manifest: transaction_manifest.as_ref().to_owned(),
            version: version.into(),
            blobs: blobs.into(),
            message: message.into(),
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionTransactionItems;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
