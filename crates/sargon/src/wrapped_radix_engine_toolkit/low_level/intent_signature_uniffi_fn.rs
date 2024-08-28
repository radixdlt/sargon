use crate::prelude::*;

#[uniffi::export]
pub fn new_intent_signature_from_signature_with_public_key(
    signature_with_public_key: SignatureWithPublicKey,
) -> IntentSignature {
    IntentSignature::from(signature_with_public_key)
}

#[uniffi::export]
pub fn intent_signature_get_signature_with_public_key(
    intent_signature: &IntentSignature,
) -> SignatureWithPublicKey {
    intent_signature.secret_magic
}

#[uniffi::export]
pub fn new_intent_signature_sample() -> IntentSignature {
    IntentSignature::sample()
}

#[uniffi::export]
pub fn new_intent_signature_sample_other() -> IntentSignature {
    IntentSignature::sample_other()
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
