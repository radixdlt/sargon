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
            "2150c2f6b6c496d197ae03afb23f6adf23b275c675394f23786250abd006d5a2c7543566403cb414f70d0e229b0a9b55b4c74f42fc38cdf1aba2307f97686f0b"
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
