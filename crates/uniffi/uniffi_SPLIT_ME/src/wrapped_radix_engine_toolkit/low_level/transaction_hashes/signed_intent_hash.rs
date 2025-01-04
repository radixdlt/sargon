use crate::prelude::*;

decl_tx_hash!(
    /// A Signed Intent Hash is a bech32 encoded string starting with `"signedintent_"
    SignedTransactionIntentHash,
);

#[uniffi::export]
pub fn new_signed_intent_hash_sample() -> SignedTransactionIntentHash {
    InternalSignedTransactionIntentHash::sample().into()
}

#[uniffi::export]
pub fn new_signed_intent_hash_sample_other() -> SignedTransactionIntentHash {
    InternalSignedTransactionIntentHash::sample_other().into()
}
