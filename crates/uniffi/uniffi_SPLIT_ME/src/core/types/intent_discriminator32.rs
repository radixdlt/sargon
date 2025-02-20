pub use crate::prelude::*;
use sargon::IntentDiscriminator32 as InternalNonce;

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct IntentDiscriminator32 {
    pub secret_magic: u32,
}

impl IntentDiscriminator32 {
    pub fn into_internal(&self) -> InternalNonce {
        self.clone().into()
    }
}

impl From<InternalNonce> for IntentDiscriminator32 {
    fn from(internal: InternalNonce) -> Self {
        Self {
            secret_magic: internal.0,
        }
    }
}

impl From<IntentDiscriminator32> for InternalNonce {
    fn from(val: IntentDiscriminator32) -> Self {
        InternalNonce(val.secret_magic)
    }
}

#[uniffi::export]
pub fn new_nonce_random() -> IntentDiscriminator32 {
    InternalNonce::random().into()
}

#[uniffi::export]
pub fn new_nonce_from_u32(value: u32) -> IntentDiscriminator32 {
    InternalNonce::from(value).into()
}

#[uniffi::export]
pub fn new_nonce_sample() -> IntentDiscriminator32 {
    InternalNonce::sample().into()
}

#[uniffi::export]
pub fn new_nonce_sample_other() -> IntentDiscriminator32 {
    InternalNonce::sample_other().into()
}

#[uniffi::export]
pub fn nonce_get_value(intent_discriminator: IntentDiscriminator32) -> u32 {
    u32::from(intent_discriminator.into_internal())
}

decl_conversion_tests_for!(IntentDiscriminator32);
