use crate::prelude::*;

#[derive(
    Clone, PartialEq, Eq, Serialize, Deserialize, Debug, uniffi::Record,
)]
pub struct UnvalidatedTransactionManifest {
    #[serde(rename = "transactionManifest")]
    pub transaction_manifest_string: String,

    #[serde(default)]
    pub blobs: Blobs,
}

impl UnvalidatedTransactionManifest {
    pub fn new(
        transaction_manifest_string: impl AsRef<str>,
        blobs: impl Into<Blobs>,
    ) -> Self {
        Self {
            transaction_manifest_string: transaction_manifest_string
                .as_ref()
                .to_owned(),
            blobs: blobs.into(),
        }
    }
}

impl From<TransactionManifest> for UnvalidatedTransactionManifest {
    fn from(transaction_manifest: TransactionManifest) -> Self {
        Self {
            transaction_manifest_string: transaction_manifest
                .instructions_string(),
            blobs: transaction_manifest.blobs().clone(),
        }
    }
}

impl HasSampleValues for UnvalidatedTransactionManifest {
    fn sample() -> Self {
        Self::new(
            TransactionManifest::sample().instructions_string(),
            Blobs::default(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            TransactionManifest::sample_other().instructions_string(),
            Blobs::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = UnvalidatedTransactionManifest;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn transaction_manifest() {
        let transaction_manifest =
            TransactionManifest::try_from((SUT::sample(), NetworkID::Mainnet));
        pretty_assertions::assert_eq!(
            transaction_manifest,
            TransactionManifest::new(
                TransactionManifest::sample().instructions_string(),
                NetworkID::Mainnet,
                Blobs::default(),
            )
        );
    }
}
