use crate::prelude::*;
use manifests::StaticallyAnalyzableManifest;
use sargon::TransactionManifest as InternalTransactionManifest;

#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct TransactionManifest {
    pub raw_manifest: BagOfBytes,
    pub network_id: NetworkID,
}

impl TransactionManifest {
    pub fn into_internal(&self) -> InternalTransactionManifest {
        self.clone().into()
    }
}

impl From<TransactionManifest> for InternalTransactionManifest {
    fn from(val: TransactionManifest) -> Self {
        let scrypto_manifest =
            RET_from_payload_bytes_manifest_v1(val.raw_manifest.to_vec())
                .unwrap();
        (scrypto_manifest, val.network_id.into())
            .try_into()
            .unwrap()
    }
}

impl From<InternalTransactionManifest> for TransactionManifest {
    fn from(manifest: InternalTransactionManifest) -> Self {
        Self {
            raw_manifest: RET_to_payload_bytes_manifest_v1(
                &manifest.scrypto_manifest(),
            )
            .unwrap()
            .into(),
            network_id: manifest.network_id().into(),
        }
    }
}

decl_conversion_tests_for!(TransactionManifest);

#[uniffi::export]
pub fn is_access_controller_timed_recovery_manifest(
    manifest: &TransactionManifest,
) -> bool {
    manifest
        .into_internal()
        .is_access_controller_timed_recovery_manifest()
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
    .into_result()
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
    .into_result()
}

#[uniffi::export]
pub fn transaction_manifest_string(manifest: &TransactionManifest) -> String {
    manifest.into_internal().manifest_string()
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
    let internal_manifest = manifest.into_internal();
    internal_manifest.summary().unwrap().into()
}

#[uniffi::export]
pub fn transaction_manifest_involved_resource_addresses(
    manifest: &TransactionManifest,
) -> Vec<ResourceAddress> {
    manifest
        .into_internal()
        .involved_resource_addresses()
        .into_type()
}

#[uniffi::export]
pub fn transaction_manifest_involved_pool_addresses(
    manifest: &TransactionManifest,
) -> Vec<PoolAddress> {
    manifest
        .into_internal()
        .involved_pool_addresses()
        .into_type()
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
