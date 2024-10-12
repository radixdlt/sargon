use crate::prelude::*;
use sargon::BagOfBytes as InternalBagOfBytes;
use sargon::Blob as InternalBlob;

/// Blob is a wrapper a bag of bytes
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct Blob {
    pub(crate) value: BagOfBytes,
}

impl From<InternalBlob> for Blob {
    fn from(value: InternalBlob) -> Self {
        Self {
            value: value.0.into(),
        }
    }
}

impl Into<InternalBlob> for Blob {
    fn into(self) -> InternalBlob {
        InternalBlob(self.value.into())
    }
}

#[uniffi::export]
pub fn new_blob_from_bytes(bytes: BagOfBytes) -> Blob {
    InternalBlob::from(bytes.into_internal()).into()
}

#[uniffi::export]
pub fn blob_to_bytes(blob: &Blob) -> BagOfBytes {
    blob.value.clone()
}

#[uniffi::export]
pub fn blob_to_string(blob: &Blob) -> String {
    blob.into_internal().to_string()
}
