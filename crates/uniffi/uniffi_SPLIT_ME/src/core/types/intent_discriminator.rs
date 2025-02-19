pub use crate::prelude::*;
use sargon::IntentDiscriminator as InternalIntentDiscriminator;

/// A random number generated part of an intent header,
/// ensuring every intent is unique even though its
/// transaction manifest might be equal. This intent discriminator is
/// generated by wallets for incoming intents.
///
/// `IntentDiscriminator` is similar to the `IntentDisciminator32` used in `TransactionHeader`.
#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct IntentDiscriminator {
    pub secret_magic: u64,
}

impl IntentDiscriminator {
    pub fn into_internal(&self) -> InternalIntentDiscriminator {
        self.clone().into()
    }
}

impl From<InternalIntentDiscriminator> for IntentDiscriminator {
    fn from(internal: InternalIntentDiscriminator) -> Self {
        Self {
            secret_magic: internal.0,
        }
    }
}

impl From<IntentDiscriminator> for InternalIntentDiscriminator {
    fn from(val: IntentDiscriminator) -> Self {
        InternalIntentDiscriminator(val.secret_magic)
    }
}

#[uniffi::export]
pub fn new_intent_discriminator_random() -> IntentDiscriminator {
    InternalIntentDiscriminator::random().into()
}

#[uniffi::export]
pub fn new_intent_discriminator_from_u64(value: u64) -> IntentDiscriminator {
    InternalIntentDiscriminator::from(value).into()
}

#[uniffi::export]
pub fn new_intent_discriminator_sample() -> IntentDiscriminator {
    InternalIntentDiscriminator::sample().into()
}

#[uniffi::export]
pub fn new_intent_discriminator_sample_other() -> IntentDiscriminator {
    InternalIntentDiscriminator::sample_other().into()
}

#[uniffi::export]
pub fn intent_discriminator_get_value(
    intent_discriminator: IntentDiscriminator,
) -> u64 {
    u64::from(intent_discriminator.into_internal())
}
