use crate::prelude::*;
use sargon::SignedAuthIntent as InternalSignedAuthIntent;

#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct SignedAuthIntent {
    pub intent: AuthIntent,
    pub intent_signatures: IntentSignatures,
}

#[uniffi::export]
pub fn new_signed_auth_intent_sample() -> SignedAuthIntent {
    InternalSignedAuthIntent::sample().into()
}

#[uniffi::export]
pub fn new_signed_auth_intent_sample_other() -> SignedAuthIntent {
    InternalSignedAuthIntent::sample_other().into()
}
