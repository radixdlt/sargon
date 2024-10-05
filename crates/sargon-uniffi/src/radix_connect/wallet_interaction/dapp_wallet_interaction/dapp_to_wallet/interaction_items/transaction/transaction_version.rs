use crate::prelude::*;
use sargon::TXVersion as InternalTXVersion;

uniffi::custom_newtype!(TXVersion, u64);
#[derive(Debug, Clone, PartialEq)]
pub struct TXVersion(u64);

impl From<InternalTXVersion> for TXVersion {
    fn from(value: InternalTXVersion) -> Self {
        Self(value.0)
    }
}

impl Into<InternalTXVersion> for TXVersion {
    fn into(self) -> InternalTXVersion {
        InternalTXVersion(self.0)
    }
}

