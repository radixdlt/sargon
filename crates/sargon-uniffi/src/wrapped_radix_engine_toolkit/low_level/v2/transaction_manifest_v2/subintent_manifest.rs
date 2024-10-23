use crate::prelude::*;
use sargon::SubintentManifest as InternalSubintentManifest;

#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct SubintentManifest {
    pub instructions: InstructionsV2,
    pub blobs: Blobs,
    pub children: ChildIntents,
}

#[uniffi::export]
pub fn subintent_manifest_string(manifest: &SubintentManifest) -> String {
    manifest.into_internal().manifest_string()
}

#[uniffi::export]
pub fn subintent_manifest_summary(
    manifest: &SubintentManifest,
) -> Option<ManifestSummary> {
    manifest.into_internal().summary().map(|s| s.into())
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
    manifest.blobs.clone()
}

#[uniffi::export]
pub fn new_subintent_manifest_sample() -> SubintentManifest {
    InternalSubintentManifest::sample().into()
}

#[uniffi::export]
pub fn new_subintent_manifest_sample_other() -> SubintentManifest {
    InternalSubintentManifest::sample_other().into()
}
