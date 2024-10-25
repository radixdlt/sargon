use crate::prelude::*;
use sargon::U30 as InternalU30;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct U30 {
    pub secret_magic: u32,
}

#[uniffi::export]
pub fn new_u30_sample() -> U30 {
    InternalU30::sample().into()
}

#[uniffi::export]
pub fn new_u30_sample_other() -> U30 {
    InternalU30::sample_other().into()
}

#[uniffi::export]
pub fn new_u30(value: u32) -> Result<U30> {
    InternalU30::try_from(value).into_result()
}

#[uniffi::export]
pub fn u30_get_value(u30: U30) -> u32 {
    u30.secret_magic
}

impl From<U30> for InternalU30 {
    fn from(value: U30) -> InternalU30 {
        InternalU30::try_from(value.secret_magic)
            .expect("InternalConversion should always work")
    }
}

impl From<InternalU30> for U30 {
    fn from(value: InternalU30) -> U30 {
        U30 {
            secret_magic: value.0.into(),
        }
    }
}
