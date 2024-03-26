use crate::prelude::*;

#[uniffi::export]
pub fn new_signature_with_public_key_sample() -> SignatureWithPublicKey {
    SignatureWithPublicKey::sample()
}

#[uniffi::export]
pub fn new_signature_with_public_key_sample_other() -> SignatureWithPublicKey {
    SignatureWithPublicKey::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SignatureWithPublicKey;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_signature_with_public_key_sample(),
                new_signature_with_public_key_sample_other(),
                // duplicates should get removed
                new_signature_with_public_key_sample(),
                new_signature_with_public_key_sample_other(),
            ])
            .len(),
            2
        );
    }
}
