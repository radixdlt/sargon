use crate::prelude::*;
use sargon::AuthIntentHash as InternalAuthIntentHash;

#[derive(
    Clone, PartialEq, Eq, std::hash::Hash, InternalConversion, uniffi::Record,
)]
pub struct AuthIntentHash {
    pub payload: BagOfBytes,
}

#[uniffi::export]
pub fn new_auth_intent_hash_sample() -> AuthIntentHash {
    InternalAuthIntentHash::sample().into()
}

#[uniffi::export]
pub fn new_auth_intent_hash_sample_other() -> AuthIntentHash {
    InternalAuthIntentHash::sample_other().into()
}
