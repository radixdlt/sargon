use crate::prelude::*;
use sargon::Blobs as InternalBlobs;

/// Vec of Blobs
#[derive(
    Clone, PartialEq, Eq, InternalConversion,  uniffi::Record,
)]
pub struct Blobs {
    pub(crate) secret_magic: Vec<Blob>,
}

impl From<InternalBlobs> for Blobs {
    fn from(value: InternalBlobs) -> Self {
        Self {
            secret_magic: value.0.into(),
        }
    }
}

impl Into<InternalBlobs> for Blobs {
    fn into(self) -> InternalBlobs {
        InternalBlobs(self.secret_magic.into())
    }
}

#[uniffi::export]
pub fn blobs_list_of_blobs(blobs: &Blobs) -> Vec<Blob> {
    blobs.into_internal().blobs().map(Blob::from).collect()
}

#[uniffi::export]
pub fn new_blobs_from_blob_list(blobs: Vec<Blob>) -> Blobs {
    InternalBlobs::new(blobs.into_iter().map(Into::into).collect()).into()
}

#[uniffi::export]
pub fn new_blobs_sample() -> Blobs {
    InternalBlobs::sample().into()
}

#[uniffi::export]
pub fn new_blobs_sample_other() -> Blobs {
    InternalBlobs::sample_other().into()
}

