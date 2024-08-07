use crate::prelude::*;

#[uniffi::export]
pub fn new_ed25519_signature_sample() -> Ed25519Signature {
    Ed25519Signature::sample()
}

#[uniffi::export]
pub fn new_ed25519_signature_sample_other() -> Ed25519Signature {
    Ed25519Signature::sample_other()
}

#[uniffi::export]
pub fn new_ed25519_signature_from_exactly_64_bytes(
    bytes: Exactly64Bytes,
) -> Ed25519Signature {
    Ed25519Signature::from(bytes)
}

#[uniffi::export]
pub fn new_ed25519_signature_from_bytes(
    bytes: BagOfBytes,
) -> Result<Ed25519Signature> {
    Ed25519Signature::try_from(bytes)
}

#[uniffi::export]
pub fn ed25519_signature_to_string(signature: &Ed25519Signature) -> String {
    signature.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Ed25519Signature;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_ed25519_signature_sample(),
                new_ed25519_signature_sample_other(),
                // duplicates should get removed
                new_ed25519_signature_sample(),
                new_ed25519_signature_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            ed25519_signature_to_string(&SUT::sample()),
            "fc6a4a15516b886b10f26777094cb1abdccb213c9ebdea7a4bceb83b6fcba50fea181b0136ee5659c3dfae5f771e5b6e6f9abbaa3f0435df0be1f732be965103"
        )
    }

    #[test]
    fn test_from_exactly_64_bytes() {
        let sut = SUT::sample();
        assert_eq!(new_ed25519_signature_from_exactly_64_bytes(sut.bytes), sut)
    }

    #[test]
    fn test_from_bag_of_bytes() {
        let sut = SUT::sample();
        assert_eq!(
            new_ed25519_signature_from_bytes(BagOfBytes::from(sut.to_bytes()))
                .unwrap(),
            sut
        )
    }
}
