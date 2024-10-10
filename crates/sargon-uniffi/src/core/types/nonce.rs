pub use crate::prelude::*;
use sargon::Nonce as InternalNonce;

#[derive(
    Clone,
    
    PartialEq,
    Eq,
    Hash,
    
    
     uniffi::Record,
)]
pub struct Nonce {
    value: u32,
}

impl From<InternalNonce> for Nonce {
    fn from(value: InternalNonce) -> Self {
        Self { value: value.0 }
    }
}

impl Into<InternalNonce> for Nonce {
    fn into(self) -> InternalNonce {
        InternalNonce(self.value)
    }
}

#[uniffi::export]
pub fn new_nonce_random() -> Nonce {
    InternalNonce::random().into()
}

#[uniffi::export]
pub fn new_nonce_from_u32(value: u32) -> Nonce {
    InternalNonce::from(value).into()
}

#[uniffi::export]
pub fn new_nonce_sample() -> Nonce {
    InternalNonce::sample().into()
}

#[uniffi::export]
pub fn new_nonce_sample_other() -> Nonce {
    InternalNonce::sample_other().into()
}

#[uniffi::export]
pub fn nonce_get_value(nonce: Nonce) -> u32 {
    u32::from(nonce.into())
}

