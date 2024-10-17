use crate::prelude::*;
use sargon::IntentSignature as InternalIntentSignature;

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct IntentSignature {
    pub(crate) value: SignatureWithPublicKey,
}

impl IntentSignature {
    pub fn into_internal(&self) -> InternalIntentSignature {
        self.clone().into()
    }
}

impl From<InternalIntentSignature> for IntentSignature {
    fn from(value: InternalIntentSignature) -> Self {
        Self {
            value: value.0.into(),
        }
    }
}

impl Into<InternalIntentSignature> for IntentSignature {
    fn into(self) -> InternalIntentSignature {
        InternalIntentSignature(self.value.into())
    }
}

#[uniffi::export]
pub fn new_intent_signature_from_signature_with_public_key(
    signature_with_public_key: SignatureWithPublicKey,
) -> IntentSignature {
    InternalIntentSignature::from(signature_with_public_key.into_internal())
        .into()
}

#[uniffi::export]
pub fn intent_signature_get_signature_with_public_key(
    intent_signature: &IntentSignature,
) -> SignatureWithPublicKey {
    intent_signature.value.clone()
}

#[uniffi::export]
pub fn new_intent_signature_sample() -> IntentSignature {
    InternalIntentSignature::sample().into()
}

#[uniffi::export]
pub fn new_intent_signature_sample_other() -> IntentSignature {
    InternalIntentSignature::sample_other().into()
}

decl_conversion_tests_for!(IntentSignature);
