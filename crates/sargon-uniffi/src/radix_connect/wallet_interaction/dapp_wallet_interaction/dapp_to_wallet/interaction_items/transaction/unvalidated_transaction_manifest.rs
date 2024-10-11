use crate::prelude::*;
use sargon::UnvalidatedTransactionManifest as InternalUnvalidatedTransactionManifest;

#[derive(
    Clone, PartialEq, Eq, InternalConversion, uniffi::Record,
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
    InternalUnvalidatedTransactionManifest::from(transaction_manifest.into_internal()).into()
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

