use crate::prelude::*;
use sargon::AesGcm256 as InternalAesGcm256;

/// AES GCM 256 encryption
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct AesGcm256 {}

impl From<InternalAesGcm256> for AesGcm256 {
    fn from(_: InternalAesGcm256) -> Self {
        Self {}
    }
}

impl Into<InternalAesGcm256> for AesGcm256 {
    fn into(self) -> InternalAesGcm256 {
        InternalAesGcm256 {}
    }
}
