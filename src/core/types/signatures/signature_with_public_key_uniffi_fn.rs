use crate::prelude::*;

#[uniffi::export]
pub fn new_signature_with_public_key_sample() -> SignatureWithPublicKey {
    SignatureWithPublicKey::sample()
}

#[uniffi::export]
pub fn new_signature_with_public_key_sample_other() -> SignatureWithPublicKey {
    SignatureWithPublicKey::sample_other()
}

#[uniffi::export]
pub fn signature_with_public_key_get_public_key(
    signature_with_public_key: &SignatureWithPublicKey,
) -> PublicKey {
    signature_with_public_key.public_key()
}

#[uniffi::export]
pub fn signature_with_public_key_get_signature(
    signature_with_public_key: &SignatureWithPublicKey,
) -> Signature {
    signature_with_public_key.signature()
}

#[uniffi::export]
pub fn signature_with_public_key_is_valid(
    signature_with_public_key: &SignatureWithPublicKey,
    for_hash: &Hash,
) -> bool {
    signature_with_public_key.is_valid_for_hash(for_hash)
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

    #[test]
    fn get_public_key() {
        assert_eq!(
            signature_with_public_key_get_public_key(&SUT::sample()),
            SUT::sample().public_key()
        )
    }

    #[test]
    fn get_signature() {
        assert_eq!(
            signature_with_public_key_get_signature(&SUT::sample()),
            SUT::sample().signature()
        )
    }

    #[test]
    fn is_valid() {
        let private_key = HierarchicalDeterministicPrivateKey::sample();
        let msg = Hash::sample();
        let sut = private_key.sign(&msg);
        assert!(signature_with_public_key_is_valid(&sut, &msg));
    }
}
