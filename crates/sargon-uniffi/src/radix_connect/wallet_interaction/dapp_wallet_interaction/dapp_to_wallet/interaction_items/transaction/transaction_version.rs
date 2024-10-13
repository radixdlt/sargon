use crate::prelude::*;
use sargon::TXVersion as InternalTXVersion;

uniffi::custom_newtype!(TXVersion, u64);

#[derive(Clone, PartialEq, InternalConversionV2)]
pub struct TXVersion(pub u64);
