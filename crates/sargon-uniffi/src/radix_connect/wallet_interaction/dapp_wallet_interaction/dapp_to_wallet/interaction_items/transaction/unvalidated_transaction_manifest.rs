use crate::prelude::*;
use sargon::UnvalidatedTransactionManifest as InternalUnvalidatedTransactionManifest;

#[derive(
    Clone, PartialEq, Eq, Debug, uniffi::Record,
)]
pub struct UnvalidatedTransactionManifest {
    pub transaction_manifest_string: String,
    pub blobs: Blobs,
}

impl From<InternalUnvalidatedTransactionManifest> for UnvalidatedTransactionManifest {
    fn from(value: InternalUnvalidatedTransactionManifest) -> Self {
        Self {
            transaction_manifest_string: value.transaction_manifest_string,
            blobs: value.blobs.into(),
        }
    }
}

impl Into<InternalUnvalidatedTransactionManifest> for UnvalidatedTransactionManifest {
    fn into(self) -> InternalUnvalidatedTransactionManifest {
        InternalUnvalidatedTransactionManifest {
            transaction_manifest_string: self.transaction_manifest_string,
            blobs: self.blobs.into(),
        }
    }
}

#[uniffi::export]
pub fn new_unvalidated_transaction_manifest_from_transaction_manifest(
    transaction_manifest: TransactionManifest,
) -> UnvalidatedTransactionManifest {
    InternalUnvalidatedTransactionManifest::from(transaction_manifest.into()).into()
}

#[uniffi::export]
pub fn new_unvalidated_transaction_manifest_sample(
) -> UnvalidatedTransactionManifest {
    InternalUnvalidatedTransactionManifest::sample().into()
}

#[uniffi::export]
pub fn new_unvalidated_transaction_manifest_sample_other(
) -> UnvalidatedTransactionManifest {
    InternalUnvalidatedTransactionManifest::sample_other().into()
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[test]
    fn sample_values() {
        assert_ne!(
            new_unvalidated_transaction_manifest_sample(),
            new_unvalidated_transaction_manifest_sample_other(),
        );
    }

    #[test]
    fn test_new_unvalidated_transaction_manifest_from_transaction_manifest() {
        let transaction_manifest = TransactionManifest::sample();
        let sut =
            new_unvalidated_transaction_manifest_from_transaction_manifest(
                transaction_manifest.clone(),
            );
        pretty_assertions::assert_eq!(
            sut.transaction_manifest_string,
            transaction_manifest.instructions_string()
        );
        pretty_assertions::assert_eq!(
            sut.blobs,
            transaction_manifest.blobs().clone()
        );
    }
}
