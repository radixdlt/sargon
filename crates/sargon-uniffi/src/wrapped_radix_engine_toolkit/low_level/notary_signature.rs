use crate::prelude::*;
use sargon::NotarySignature as InternalNotarySignature;

#[derive(Clone, Eq, Hash, PartialEq, uniffi::Record)]
pub struct NotarySignature {
    pub(crate) secret_magic: Signature,
}

impl NotarySignature {
    pub fn into_internal(&self) -> InternalNotarySignature {
        self.clone().into()
    }
}

impl From<InternalNotarySignature> for NotarySignature {
    fn from(internal: InternalNotarySignature) -> Self {
        Self {
            secret_magic: internal.0.into(),
        }
    }
}

impl Into<InternalNotarySignature> for NotarySignature {
    fn into(self) -> InternalNotarySignature {
        InternalNotarySignature(self.secret_magic.into())
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
    NotarySignature {
        secret_magic: signature,
    }
}

#[uniffi::export]
pub fn notary_signature_get_signature(
    notary_signature: &NotarySignature,
) -> Signature {
    notary_signature.secret_magic.clone()
}

#[uniffi::export]
pub fn android_notarize_hash_with_private_key_bytes(
    private_key_bytes: Exactly32Bytes,
    signed_intent_hash: &SignedIntentHash,
) -> Result<NotarySignature> {
    sargon::android_notarize_hash_with_private_key_bytes(
        private_key_bytes.into_internal(),
        &signed_intent_hash.into_internal(),
    )
    .into_result()
}

#[uniffi::export]
pub fn android_sign_hash_with_private_key_bytes(
    private_key_bytes: Exactly32Bytes,
    hash: &Hash,
) -> Result<Ed25519Signature> {
    sargon::android_sign_hash_with_private_key_bytes(
        private_key_bytes.into_internal(),
        &hash.into_internal(),
    )
    .into_result()
}

decl_conversion_tests_for!(NotarySignature);
