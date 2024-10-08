use crate::prelude::*;

/// Either a Signature on `Curve25519` or `Secp256k1`
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    EnumAsInner,
    
    
    
    
    uniffi::Enum,
)]
pub enum Signature {
    Secp256k1 { value: Secp256k1Signature },
    Ed25519 { value: Ed25519Signature },
}

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
            "fc6a4a15516b886b10f26777094cb1abdccb213c9ebdea7a4bceb83b6fcba50fea181b0136ee5659c3dfae5f771e5b6e6f9abbaa3f0435df0be1f732be965103"
        )
    }

    #[test]
    fn test_to_bytes() {
        assert_eq!(
            signature_to_bytes(&SUT::sample()).to_string(),
            "fc6a4a15516b886b10f26777094cb1abdccb213c9ebdea7a4bceb83b6fcba50fea181b0136ee5659c3dfae5f771e5b6e6f9abbaa3f0435df0be1f732be965103"
        )
    }

    #[test]
    fn test_new_signature_from_bytes() {
        let bytes: BagOfBytes = "fc6a4a15516b886b10f26777094cb1abdccb213c9ebdea7a4bceb83b6fcba50fea181b0136ee5659c3dfae5f771e5b6e6f9abbaa3f0435df0be1f732be965103".parse().unwrap();
        assert_eq!(new_signature_from_bytes(bytes).unwrap(), SUT::sample())
    }
}
