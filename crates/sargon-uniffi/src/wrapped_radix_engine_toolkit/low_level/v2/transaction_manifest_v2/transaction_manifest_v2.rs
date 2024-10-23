use crate::prelude::*;
use sargon::TransactionManifestV2 as InternalTransactionManifestV2;

#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct TransactionManifestV2 {
    pub instructions: InstructionsV2,
    pub blobs: Blobs,
    pub children: ChildIntents,
}

#[uniffi::export]
pub fn transaction_manifest_string_v2(
    manifest: &TransactionManifestV2,
) -> String {
    manifest.into_internal().manifest_string()
}

#[uniffi::export]
pub fn transaction_manifest_summary_v2(
    manifest: &TransactionManifestV2,
) -> Option<ManifestSummary> {
    manifest.into_internal().summary().map(|s| s.into())
}

#[uniffi::export]
pub fn transaction_manifest_involved_resource_addresses_v2(
    manifest: &TransactionManifestV2,
) -> Vec<ResourceAddress> {
    manifest
        .into_internal()
        .involved_resource_addresses()
        .into_type()
}

#[uniffi::export]
pub fn transaction_manifest_involved_pool_addresses_v2(
    manifest: &TransactionManifestV2,
) -> Vec<PoolAddress> {
    manifest
        .into_internal()
        .involved_pool_addresses()
        .into_type()
}

#[uniffi::export]
pub fn transaction_manifest_network_id_v2(
    manifest: &TransactionManifestV2,
) -> NetworkID {
    manifest.into_internal().network_id().into()
}

#[uniffi::export]
pub fn transaction_manifest_blobs_v2(
    manifest: &TransactionManifestV2,
) -> Blobs {
    manifest.blobs.clone()
}

#[uniffi::export]
pub fn new_transaction_manifest_v2_sample() -> TransactionManifestV2 {
    InternalTransactionManifestV2::sample().into()
}

#[uniffi::export]
pub fn new_transaction_manifest_v2_sample_other() -> TransactionManifestV2 {
    InternalTransactionManifestV2::sample_other().into()
}
