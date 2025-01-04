use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct IntentSignatureOfOwner {
    pub owner: AddressOfAccountOrPersona,
    pub intent_signature: IntentSignature,
}

impl IntentSignatureOfOwner {
    pub fn new(
        owner: AddressOfAccountOrPersona,
        intent_signature: IntentSignature,
    ) -> Self {
        Self {
            owner,
            intent_signature,
        }
    }
}
