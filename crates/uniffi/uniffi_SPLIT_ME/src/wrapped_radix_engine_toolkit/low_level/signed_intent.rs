use crate::prelude::*;
use sargon::SignedIntent as InternalSignedIntent;

#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct SignedIntent {
    intent: TransactionIntent,
    pub intent_signatures: IntentSignatures,
}

#[uniffi::export]
pub fn signed_intent_hash(
    signed_intent: &SignedIntent,
) -> SignedTransactionIntentHash {
    signed_intent.into_internal().hash().into()
}

#[uniffi::export]
pub fn new_signed_intent_sample() -> SignedIntent {
    InternalSignedIntent::sample().into()
}

#[uniffi::export]
pub fn new_signed_intent_sample_other() -> SignedIntent {
    InternalSignedIntent::sample_other().into()
}
