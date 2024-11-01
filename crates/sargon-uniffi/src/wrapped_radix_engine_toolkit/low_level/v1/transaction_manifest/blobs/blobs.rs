use crate::prelude::*;
use sargon::Blobs as InternalBlobs;

/// Vec of Blobs
#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct Blobs {
    pub secret_magic: Vec<Blob>,
}

impl Blobs {
    pub fn into_internal(&self) -> InternalBlobs {
        self.clone().into()
    }
}

impl From<InternalBlobs> for Blobs {
    fn from(internal: InternalBlobs) -> Self {
        Self {
            secret_magic: internal.0.into_type(),
        }
    }
}

impl From<Blobs> for InternalBlobs {
    fn from(val: Blobs) -> Self {
        InternalBlobs::from_vec(val.secret_magic.into_internal())
    }
}

#[uniffi::export]
pub fn blobs_list_of_blobs(blobs: &Blobs) -> Vec<Blob> {
    blobs.secret_magic.clone()
}

#[uniffi::export]
pub fn new_blobs_from_blob_list(blobs: Vec<Blob>) -> Blobs {
    Blobs {
        secret_magic: blobs,
    }
}

#[uniffi::export]
pub fn new_blobs_sample() -> Blobs {
    InternalBlobs::sample().into()
}

#[uniffi::export]
pub fn new_blobs_sample_other() -> Blobs {
    InternalBlobs::sample_other().into()
}

decl_conversion_tests_for!(Blobs);
