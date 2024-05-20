use crate::prelude::*;

#[uniffi::export]
pub fn new_signature_sample() -> Signature {
    Signature::sample()
}

#[uniffi::export]
pub fn new_signature_sample_other() -> Signature {
    Signature::sample_other()
}

#[uniffi::export]
pub fn new_signature_from_bytes(bytes: BagOfBytes) -> Result<Signature> {
    Signature::try_from(bytes)
}

#[uniffi::export]
pub fn signature_to_string(signature: &Signature) -> String {
    signature.to_string()
}

#[uniffi::export]
pub fn signature_to_bytes(signature: &Signature) -> BagOfBytes {
    BagOfBytes::from(signature.to_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Signature;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_signature_sample(),
                new_signature_sample_other(),
                // duplicates should get removed
                new_signature_sample(),
                new_signature_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            signature_to_string(&SUT::sample()),
            "2150c2f6b6c496d197ae03afb23f6adf23b275c675394f23786250abd006d5a2c7543566403cb414f70d0e229b0a9b55b4c74f42fc38cdf1aba2307f97686f0b"
        )
    }

    #[test]
    fn test_to_bytes() {
        assert_eq!(
            signature_to_bytes(&SUT::sample()).to_string(),
            "2150c2f6b6c496d197ae03afb23f6adf23b275c675394f23786250abd006d5a2c7543566403cb414f70d0e229b0a9b55b4c74f42fc38cdf1aba2307f97686f0b"
        )
    }

    #[test]
    fn test_new_signature_from_bytes() {
        let bytes: BagOfBytes = "2150c2f6b6c496d197ae03afb23f6adf23b275c675394f23786250abd006d5a2c7543566403cb414f70d0e229b0a9b55b4c74f42fc38cdf1aba2307f97686f0b".parse().unwrap();
        assert_eq!(new_signature_from_bytes(bytes).unwrap(), SUT::sample())
    }
}
