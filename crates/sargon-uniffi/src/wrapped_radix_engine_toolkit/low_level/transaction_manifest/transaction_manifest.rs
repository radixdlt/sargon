use crate::prelude::*;
use sargon::TransactionManifest as InternalTransactionManifest;

#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct TransactionManifest {
    pub instructions: Instructions,
    pub blobs: Blobs,
}

impl From<InternalTransactionManifest> for TransactionManifest {
    fn from(value: InternalTransactionManifest) -> Self {
        Self {
            instructions: value.instructions.into(),
            blobs: value.blobs.into(),
        }
    }
}

impl Into<InternalTransactionManifest> for TransactionManifest {
    fn into(self) -> InternalTransactionManifest {
        InternalTransactionManifest {
            instructions: self.instructions.into(),
            blobs: self.blobs.into(),
        }
    }
}

#[uniffi::export]
pub fn new_transaction_manifest_from_instructions_string_and_blobs(
    instructions_string: String,
    network_id: NetworkID,
    blobs: Blobs,
) -> Result<TransactionManifest> {
    InternalTransactionManifest::new(
        instructions_string,
        network_id.into(),
        blobs.into(),
    )
    .map_result()
}

#[uniffi::export]
pub fn new_transaction_manifest_from_unvalidated_transaction_manifest(
    unvalidated_transaction_manifest: UnvalidatedTransactionManifest,
    network_id: NetworkID,
) -> Result<TransactionManifest> {
    InternalTransactionManifest::try_from((
        unvalidated_transaction_manifest.into_internal(),
        network_id.into(),
    ))
    .map_result()
}

#[uniffi::export]
pub fn transaction_manifest_instructions_string(
    manifest: &TransactionManifest,
) -> String {
    manifest.into_internal().instructions_string()
}

#[uniffi::export]
pub fn transaction_manifest_summary(
    manifest: &TransactionManifest,
) -> ManifestSummary {
    manifest.into_internal().summary().into()
}

#[uniffi::export]
pub fn transaction_manifest_involved_resource_addresses(
    manifest: &TransactionManifest,
) -> Vec<ResourceAddress> {
    manifest
        .into_internal()
        .involved_resource_addresses()
        .into_iter()
        .map(|x| x.into())
        .collect()
}

#[uniffi::export]
pub fn transaction_manifest_involved_pool_addresses(
    manifest: &TransactionManifest,
) -> Vec<PoolAddress> {
    manifest
        .into_internal()
        .involved_pool_addresses()
        .into_iter()
        .map(|x| x.into())
        .collect()
}

#[uniffi::export]
pub fn transaction_manifest_execution_summary(
    manifest: &TransactionManifest,
    engine_toolkit_receipt: String,
) -> Result<ExecutionSummary> {
    manifest
        .into_internal()
        .execution_summary(engine_toolkit_receipt)
        .map_result()
}

#[uniffi::export]
pub fn transaction_manifest_network_id(
    manifest: &TransactionManifest,
) -> NetworkID {
    manifest.into_internal().network_id().into()
}

#[uniffi::export]
pub fn transaction_manifest_blobs(manifest: &TransactionManifest) -> Blobs {
    manifest.into_internal().blobs().clone().into()
}

#[uniffi::export]
pub fn new_transaction_manifest_sample() -> TransactionManifest {
    InternalTransactionManifest::sample().into()
}

#[uniffi::export]
pub fn new_transaction_manifest_sample_other() -> TransactionManifest {
    InternalTransactionManifest::sample_other().into()
}
