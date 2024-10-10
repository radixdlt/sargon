use crate::prelude::*;
use sargon::SignedIntent as InternalSignedIntent;

#[derive(Clone,  PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct SignedIntent {
    intent: TransactionIntent,
    pub intent_signatures: IntentSignatures,
}

impl From<InternalSignedIntent> for SignedIntent {
    fn from(value: InternalSignedIntent) -> Self {
        Self {
            intent: value.intent.into(),
            intent_signatures: value.intent_signatures.into(),
        }
    }
}

impl Into<InternalSignedIntent> for SignedIntent {
    fn into(self) -> InternalSignedIntent {
        InternalSignedIntent {
            intent: self.intent.into(),
            intent_signatures: self.intent_signatures.into(),
        }
    }
}

#[uniffi::export]
pub fn signed_intent_hash(signed_intent: &SignedIntent) -> SignedIntentHash {
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

