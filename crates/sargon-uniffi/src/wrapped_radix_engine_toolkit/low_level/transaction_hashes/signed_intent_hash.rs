use crate::prelude::*;

decl_tx_hash!(
    /// A Signed Intent Hash is a bech32 encoded string starting with `"signedintent_"
    SignedIntent,
);

#[uniffi::export]
pub fn new_signed_intent_hash_sample() -> SignedIntentHash {
    InternalSignedIntentHash::sample().into()
}

#[uniffi::export]
pub fn new_signed_intent_hash_sample_other() -> SignedIntentHash {
    InternalSignedIntentHash::sample_other().into()
}
