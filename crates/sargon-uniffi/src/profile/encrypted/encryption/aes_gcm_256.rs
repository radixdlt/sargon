use crate::prelude::*;
use sargon::AesGcm256 as InternalAesGcm256;

/// AES GCM 256 encryption
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct AesGcm256 {}