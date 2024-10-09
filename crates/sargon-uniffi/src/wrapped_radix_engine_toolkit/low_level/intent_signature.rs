use crate::prelude::*;
use sargon::IntentSignature as InternalIntentSignature;

#[derive(
    Clone,  Debug, PartialEq, Eq, Hash,  uniffi::Record,
)]
pub struct IntentSignature {
    pub(crate) secret_magic: SignatureWithPublicKey,
}

impl From<InternalIntentSignature> for IntentSignature {
    fn from(value: InternalIntentSignature) -> Self {
        Self {
            secret_magic: value.0.into(),
        }
    }
}

impl Into<InternalIntentSignature> for IntentSignature {
    fn into(self) -> InternalIntentSignature {
        InternalIntentSignature(self.secret_magic.into())
    }
}

#[uniffi::export]
pub fn new_intent_signature_from_signature_with_public_key(
    signature_with_public_key: SignatureWithPublicKey,
) -> IntentSignature {
    InternalIntentSignature::from(signature_with_public_key.into())
        .into()
}

#[uniffi::export]
pub fn intent_signature_get_signature_with_public_key(
    intent_signature: &IntentSignature,
) -> SignatureWithPublicKey {
    intent_signature.secret_magic
}

#[uniffi::export]
pub fn new_intent_signature_sample() -> IntentSignature {
    InternalIntentSignature::sample().into()
}

#[uniffi::export]
pub fn new_intent_signature_sample_other() -> IntentSignature {
    InternalIntentSignature::sample_other().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = IntentSignature;

    #[test]
    fn test_intent_signature_get_signature_with_public_key() {
        assert_eq!(
            intent_signature_get_signature_with_public_key(&SUT::sample()),
            SignatureWithPublicKey::sample()
        )
    }

    #[test]
    fn test_new_intent_signature_from_signature_with_public_key() {
        assert_eq!(
            new_intent_signature_from_signature_with_public_key(
                SignatureWithPublicKey::sample()
            ),
            SUT::sample()
        )
    }

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_intent_signature_sample(),
                new_intent_signature_sample_other(),
                // duplicates should get removed
                new_intent_signature_sample(),
                new_intent_signature_sample_other(),
            ])
            .len(),
            2
        );
    }
}
