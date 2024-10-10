use crate::prelude::*;
use sargon::NotarySignature as InternalNotarySignature;

#[derive(
    Clone,
    Eq,
    Hash,
    PartialEq,
    InternalConversion,
    uniffi::Record,
)]
pub struct NotarySignature {
    pub(crate) value: Signature,
}

impl From<InternalNotarySignature> for NotarySignature {
    fn from(value: InternalNotarySignature) -> Self {
        Self {
            value: value.0.into(),
        }
    }
}

impl Into<InternalNotarySignature> for NotarySignature {
    fn into(self) -> InternalNotarySignature {
        InternalNotarySignature(self.value.into())
    }
}

#[uniffi::export]
pub fn new_notary_signature_sample() -> NotarySignature {
    InternalNotarySignature::sample().into()
}

#[uniffi::export]
pub fn new_notary_signature_sample_other() -> NotarySignature {
    InternalNotarySignature::sample_other().into()
}

#[uniffi::export]
pub fn new_notary_signature(signature: Signature) -> NotarySignature {
    NotarySignature { value: signature }
}

#[uniffi::export]
pub fn notary_signature_get_signature(
    notary_signature: &NotarySignature,
) -> Signature {
    notary_signature.value.clone()
}

#[uniffi::export]
pub fn android_notarize_hash_with_private_key_bytes(
    private_key_bytes: Exactly32Bytes,
    signed_intent_hash: &SignedIntentHash,
) -> Result<NotarySignature> {
    unimplemented!("Should be moved as actual func in  internal Sargon")
}

#[uniffi::export]
pub fn android_sign_hash_with_private_key_bytes(
    private_key_bytes: Exactly32Bytes,
    hash: &Hash,
) -> Result<Ed25519Signature> {
    unimplemented!("Should be moved as actual func in  internal Sargon")
}

