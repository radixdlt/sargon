pub use crate::prelude::*;
use sargon::IntentDisciminator32 as InternalNonce;

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct IntentDisciminator32 {
    pub secret_magic: u32,
}

impl IntentDisciminator32 {
    pub fn into_internal(&self) -> InternalNonce {
        self.clone().into()
    }
}

impl From<InternalNonce> for IntentDisciminator32 {
    fn from(internal: InternalNonce) -> Self {
        Self {
            secret_magic: internal.0,
        }
    }
}

impl From<IntentDisciminator32> for InternalNonce {
    fn from(val: IntentDisciminator32) -> Self {
        InternalNonce(val.secret_magic)
    }
}

#[uniffi::export]
pub fn new_nonce_random() -> IntentDisciminator32 {
    InternalNonce::random().into()
}

#[uniffi::export]
pub fn new_nonce_from_u32(value: u32) -> IntentDisciminator32 {
    InternalNonce::from(value).into()
}

#[uniffi::export]
pub fn new_nonce_sample() -> IntentDisciminator32 {
    InternalNonce::sample().into()
}

#[uniffi::export]
pub fn new_nonce_sample_other() -> IntentDisciminator32 {
    InternalNonce::sample_other().into()
}

#[uniffi::export]
pub fn nonce_get_value(intent_discriminator: IntentDisciminator32) -> u32 {
    u32::from(intent_discriminator.into_internal())
}

decl_conversion_tests_for!(IntentDisciminator32);
