use crate::prelude::*;
use manifests::StaticallyAnalyzableManifest;
use sargon::TransactionManifestV2 as InternalTransactionManifestV2;

#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct TransactionManifestV2 {
    pub raw_manifest: BagOfBytes,
    pub network_id: NetworkID,
}

impl TransactionManifestV2 {
    pub fn into_internal(&self) -> InternalTransactionManifestV2 {
        self.clone().into()
    }
}

impl From<TransactionManifestV2> for InternalTransactionManifestV2 {
    fn from(val: TransactionManifestV2) -> Self {
        let scrypto_manifest =
            RET_from_payload_bytes_manifest_v2(val.raw_manifest.to_vec())
                .unwrap();
        (scrypto_manifest, val.network_id.into())
            .try_into()
            .unwrap()
    }
}

impl From<InternalTransactionManifestV2> for TransactionManifestV2 {
    fn from(manifest: InternalTransactionManifestV2) -> Self {
        Self {
            raw_manifest: RET_to_payload_bytes_manifest_v2(
                &manifest.scrypto_manifest(),
            )
            .unwrap()
            .into(),
            network_id: manifest.network_id().into(),
        }
    }
}

decl_conversion_tests_for!(TransactionManifestV2);

#[uniffi::export]
pub fn transaction_manifest_string_v2(
    manifest: &TransactionManifestV2,
) -> String {
    manifest.into_internal().manifest_string()
}

#[uniffi::export]
pub fn transaction_manifest_summary_v2(
    manifest: &TransactionManifestV2,
) -> Result<ManifestSummary> {
    manifest.into_internal().summary().into_result()
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
    manifest.into_internal().blobs.into()
}

#[uniffi::export]
pub fn new_transaction_manifest_v2_sample() -> TransactionManifestV2 {
    InternalTransactionManifestV2::sample().into()
}

#[uniffi::export]
pub fn new_transaction_manifest_v2_sample_other() -> TransactionManifestV2 {
    InternalTransactionManifestV2::sample_other().into()
}
