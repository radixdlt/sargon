pub use crate::prelude::*;
use sargon::Nonce as InternalNonce;

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct Nonce {
    pub secret_magic: u32,
}

impl Nonce {
    pub fn into_internal(&self) -> InternalNonce {
        self.clone().into()
    }
}

impl From<InternalNonce> for Nonce {
    fn from(internal: InternalNonce) -> Self {
        Self {
            secret_magic: internal.0,
        }
    }
}

impl Into<InternalNonce> for Nonce {
    fn into(self) -> InternalNonce {
        InternalNonce(self.secret_magic)
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
    u32::from(nonce.into_internal())
}
