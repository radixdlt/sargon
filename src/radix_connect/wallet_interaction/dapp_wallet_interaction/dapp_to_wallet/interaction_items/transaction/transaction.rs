use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, uniffi::Record)]
pub struct DappToWalletInteractionTransactionItems {
    pub send: DappToWalletInteractionSendTransactionItem,
}

impl DappToWalletInteractionTransactionItems {
    pub fn new(send: DappToWalletInteractionSendTransactionItem) -> Self {
        Self { send }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionSendTransactionItem {
    #[serde(flatten, with = "UnvalidatedTransactionManifest")]
    pub unvalidated_manifest: UnvalidatedTransactionManifest,

    pub version: TXVersion,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl DappToWalletInteractionSendTransactionItem {
    pub fn new(
        unvalidated_manifest: impl Into<UnvalidatedTransactionManifest>,
        version: impl Into<TXVersion>,
        message: impl Into<Option<String>>,
    ) -> Self {
        Self {
            unvalidated_manifest: unvalidated_manifest.into(),
            version: version.into(),
            message: message.into(),
        }
    }
}

impl HasSampleValues for DappToWalletInteractionTransactionItems {
    fn sample() -> Self {
        Self::new(DappToWalletInteractionSendTransactionItem::sample())
    }

    fn sample_other() -> Self {
        Self::new(DappToWalletInteractionSendTransactionItem::sample_other())
    }
}

impl HasSampleValues for DappToWalletInteractionSendTransactionItem {
    fn sample() -> Self {
        Self::new(
            UnvalidatedTransactionManifest::sample(),
            TXVersion::sample(),
            "message".to_owned(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            UnvalidatedTransactionManifest::sample_other(),
            TXVersion::sample_other(),
            "message_other".to_owned(),
        )
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
