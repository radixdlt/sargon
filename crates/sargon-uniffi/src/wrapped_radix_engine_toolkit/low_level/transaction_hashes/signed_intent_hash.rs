use crate::prelude::*;
use sargon::SignedIntentHash as InternalSignedIntentHash;

decl_tx_hash!(
    /// A Signed Intent Hash is a bech32 encoded string starting with `"signedintent_"
    SignedIntent,
    "signedintent_rdx1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6sxsk6nl",
    "sign...xsk6nl",
);

#[uniffi::export]
pub fn new_signed_intent_hash_sample() -> SignedIntentHash {
    InternalSignedIntentHash::sample().into()
}

#[uniffi::export]
pub fn new_signed_intent_hash_sample_other() -> SignedIntentHash {
    InternalSignedIntentHash::sample_other().into()
}