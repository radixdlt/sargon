use crate::prelude::*;
use sargon::NotarySignature as InternalNotarySignature;

#[derive(
    Debug,
    Clone,
    
    Eq,
    Hash,
    PartialEq,
    InternalConversion,
     uniffi::Record,
)]
pub struct NotarySignature {
    pub(crate) value: Signature,
}

impl From<InternalNotarySignature> for NotarySignature {
    fn from(value: InternalNotarySignature) -> Self {
        Self {
            value: value.0.into(),
        }
    }
}

impl Into<InternalNotarySignature> for NotarySignature {
    fn into(self) -> InternalNotarySignature {
        InternalNotarySignature(self.value.into())
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
    InternalNotarySignature::from(signature).into()
}

#[uniffi::export]
pub fn notary_signature_get_signature(
    notary_signature: &NotarySignature,
) -> Signature {
    notary_signature.into_internal().secret_magic.into()
}

#[uniffi::export]
pub fn android_notarize_hash_with_private_key_bytes(
    private_key_bytes: Exactly32Bytes,
    signed_intent_hash: &SignedIntentHash,
) -> Result<NotarySignature> {
    unimplemented!("Should be moved as actual func in  internal Sargon")
}

#[uniffi::export]
pub fn android_sign_hash_with_private_key_bytes(
    private_key_bytes: Exactly32Bytes,
    hash: &Hash,
) -> Result<Ed25519Signature> {
    unimplemented!("Should be moved as actual func in  internal Sargon")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NotarySignature;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_notary_signature_sample(),
                new_notary_signature_sample_other(),
                // duplicates should get removed
                new_notary_signature_sample(),
                new_notary_signature_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn signature_roundtrip() {
        let sut = SUT::sample();
        assert_eq!(
            new_notary_signature(notary_signature_get_signature(&sut)),
            sut
        )
    }

    #[test]
    fn test_android_notarize_hash_with_private_key_bytes() {
        let sut = android_notarize_hash_with_private_key_bytes(
            Exactly32Bytes::sample(),
            &SignedIntentHash::sample(),
        )
        .unwrap();

        assert_eq!(
            "1a30347a04bc5d746b35a568330ba69c9b6ac60ef72d0a28cb63e25680e64908557d85a0e864c423ce782b5f43da3002c301045c6385b40cb013374045392404",
            sut.to_string()
        )
    }

    #[test]
    fn test_android_sign_hash_with_private_key_bytes() {
        let sut = android_sign_hash_with_private_key_bytes(
            Exactly32Bytes::sample(),
            &Hash::sample(),
        )
        .unwrap();

        assert_eq!(
            "1a30347a04bc5d746b35a568330ba69c9b6ac60ef72d0a28cb63e25680e64908557d85a0e864c423ce782b5f43da3002c301045c6385b40cb013374045392404",
            sut.to_string()
        )
    }
}
