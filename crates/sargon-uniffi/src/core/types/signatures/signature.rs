use crate::prelude::*;
use sargon::Signature as InternalSignature;

/// Either a Signature on `Curve25519` or `Secp256k1`
#[derive(
    Clone,
    
    PartialEq,
    Eq,
    Hash,
    InternalConversion,
    uniffi::Enum,
)]
pub enum Signature {
    Secp256k1 { value: Secp256k1Signature },
    Ed25519 { value: Ed25519Signature },
}

impl From<InternalSignature> for Signature {
    fn from(value: InternalSignature) -> Self {
        match value {
            InternalSignature::Secp256k1 { value } => Signature::Secp256k1 { value: value.into() },
            InternalSignature::Ed25519 { value } => Signature::Ed25519 { value: value.into() },
        }
    }
}

impl Into<InternalSignature> for Signature {
    fn into(self) -> InternalSignature {
        match self {
            Signature::Secp256k1 { value } => InternalSignature::Secp256k1 { value: value.into() },
            Signature::Ed25519 { value } => InternalSignature::Ed25519 { value value.into() },
        }
    }
}

#[uniffi::export]
pub fn new_signature_sample() -> Signature {
    InternalSignature::sample().into()
}

#[uniffi::export]
pub fn new_signature_sample_other() -> Signature {
    InternalSignature::sample_other().into()
}

#[uniffi::export]
pub fn new_signature_from_bytes(bytes: BagOfBytes) -> Result<Signature> {
    InternalSignature::try_from(bytes.into()).map_result()
}

#[uniffi::export]
pub fn signature_to_string(signature: &Signature) -> String {
    signature.into_internal().to_string()
}

#[uniffi::export]
pub fn signature_to_bytes(signature: &Signature) -> BagOfBytes {
    signature.into_internal().to_bytes().into()
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
