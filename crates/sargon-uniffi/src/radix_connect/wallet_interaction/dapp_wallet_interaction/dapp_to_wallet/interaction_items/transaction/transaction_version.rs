use crate::prelude::*;
use sargon::TXVersion as InternalTXVersion;

#[derive(Clone, PartialEq, uniffi::Record)]
pub struct TXVersion {
    pub value: u64,
}

impl From<InternalTXVersion> for TXVersion {
    fn from(value: InternalTXVersion) -> Self {
        Self { value: value.0 }
    }
}

impl Into<InternalTXVersion> for TXVersion {
    fn into(self) -> InternalTXVersion {
        InternalTXVersion(self.value)
    }
}
