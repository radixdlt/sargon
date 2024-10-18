use crate::prelude::*;
use sargon::BagOfBytes as InternalBagOfBytes;
use sargon::Blob as InternalBlob;

/// Blob is a wrapper a bag of bytes
#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct Blob {
    pub secret_magic: BagOfBytes,
}

impl Blob {
    pub fn into_internal(&self) -> InternalBlob {
        self.clone().into()
    }
}

impl From<InternalBlob> for Blob {
    fn from(internal: InternalBlob) -> Self {
        Self {
            secret_magic: internal.0.into(),
        }
    }
}

impl Into<InternalBlob> for Blob {
    fn into(self) -> InternalBlob {
        InternalBlob(self.secret_magic.into())
    }
}

#[uniffi::export]
pub fn new_blob_from_bytes(bytes: BagOfBytes) -> Blob {
    InternalBlob::from(bytes.into_internal()).into()
}

#[uniffi::export]
pub fn blob_to_bytes(blob: &Blob) -> BagOfBytes {
    blob.secret_magic.clone()
}

#[uniffi::export]
pub fn blob_to_string(blob: &Blob) -> String {
    blob.into_internal().to_string()
}

decl_conversion_tests_for!(Blob);
