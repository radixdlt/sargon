use crate::prelude::*;
use sargon::UnvalidatedSubintentManifest as InternalUnvalidatedSubintentManifest;

#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct UnvalidatedSubintentManifest {
    pub subintent_manifest_string: String,
    pub blobs: Blobs,
}

#[uniffi::export]
pub fn new_unvalidated_subintent_manifest_from_subintent_manifest(
    subintent_manifest: SubintentManifest,
) -> UnvalidatedSubintentManifest {
    InternalUnvalidatedSubintentManifest::from(
        subintent_manifest.into_internal(),
    )
    .into()
}

#[uniffi::export]
pub fn new_unvalidated_subintent_manifest_sample(
) -> UnvalidatedSubintentManifest {
    InternalUnvalidatedSubintentManifest::sample().into()
}

#[uniffi::export]
pub fn new_unvalidated_subintent_manifest_sample_other(
) -> UnvalidatedSubintentManifest {
    InternalUnvalidatedSubintentManifest::sample_other().into()
}
