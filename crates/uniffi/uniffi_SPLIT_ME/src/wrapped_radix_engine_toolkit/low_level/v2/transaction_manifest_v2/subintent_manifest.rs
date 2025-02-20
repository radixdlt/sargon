use crate::prelude::*;
use manifests::StaticallyAnalyzableManifest;
use sargon::SubintentManifest as InternalSubintentManifest;

#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct SubintentManifest {
    pub raw_manifest: BagOfBytes,
    pub network_id: NetworkID,
}

impl SubintentManifest {
    pub fn into_internal(&self) -> InternalSubintentManifest {
        self.clone().into()
    }
}

impl From<SubintentManifest> for InternalSubintentManifest {
    fn from(val: SubintentManifest) -> Self {
        let scrypto_manifest = RET_from_payload_bytes_subintent_manifest(
            val.raw_manifest.to_vec(),
        )
        .unwrap();
        (scrypto_manifest, val.network_id.into())
            .try_into()
            .unwrap()
    }
}

impl From<InternalSubintentManifest> for SubintentManifest {
    fn from(manifest: InternalSubintentManifest) -> Self {
        Self {
            raw_manifest: RET_to_payload_bytes_subintent_manifest(
                &manifest.scrypto_manifest(),
            )
            .unwrap()
            .into(),
            network_id: manifest.network_id().into(),
        }
    }
}

decl_conversion_tests_for!(SubintentManifest);

#[uniffi::export]
pub fn subintent_manifest_string(manifest: &SubintentManifest) -> String {
    manifest.into_internal().manifest_string()
}

#[uniffi::export]
pub fn subintent_manifest_summary(
    manifest: &SubintentManifest,
) -> ManifestSummary {
    let internal_manifest = manifest.into_internal();

    internal_manifest.summary().unwrap().into()
}

#[uniffi::export]
pub fn subintent_manifest_involved_resource_addresses(
    manifest: &SubintentManifest,
) -> Vec<ResourceAddress> {
    manifest
        .into_internal()
        .involved_resource_addresses()
        .into_type()
}

#[uniffi::export]
pub fn subintent_manifest_involved_pool_addresses(
    manifest: &SubintentManifest,
) -> Vec<PoolAddress> {
    manifest
        .into_internal()
        .involved_pool_addresses()
        .into_type()
}

#[uniffi::export]
pub fn subintent_manifest_network_id(
    manifest: &SubintentManifest,
) -> NetworkID {
    manifest.into_internal().network_id().into()
}

#[uniffi::export]
pub fn subintent_manifest_blobs(manifest: &SubintentManifest) -> Blobs {
    manifest.into_internal().blobs.into()
}

#[uniffi::export]
pub fn new_subintent_manifest_sample() -> SubintentManifest {
    InternalSubintentManifest::sample().into()
}

#[uniffi::export]
pub fn new_subintent_manifest_sample_other() -> SubintentManifest {
    InternalSubintentManifest::sample_other().into()
}
