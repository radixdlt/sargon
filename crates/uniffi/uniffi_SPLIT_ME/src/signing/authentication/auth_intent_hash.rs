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

#[uniffi::export]
pub fn auth_intent_hash_get_hash(auth_intent_hash: AuthIntentHash) -> Hash {
    auth_intent_hash.into_internal().hash().into()
}
