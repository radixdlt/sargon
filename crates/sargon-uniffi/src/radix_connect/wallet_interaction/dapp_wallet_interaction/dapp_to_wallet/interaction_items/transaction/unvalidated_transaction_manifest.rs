use crate::prelude::*;
use sargon::UnvalidatedTransactionManifest as InternalUnvalidatedTransactionManifest;

#[derive(Clone, PartialEq, Eq, InternalConversionV2, uniffi::Record)]
pub struct UnvalidatedTransactionManifest {
    pub transaction_manifest_string: String,
    pub blobs: Blobs,
}

#[uniffi::export]
pub fn new_unvalidated_transaction_manifest_from_transaction_manifest(
    transaction_manifest: TransactionManifest,
) -> UnvalidatedTransactionManifest {
    InternalUnvalidatedTransactionManifest::from(
        transaction_manifest.into_internal(),
    )
    .into()
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
