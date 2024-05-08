use crate::prelude::*;

#[uniffi::export]
pub fn new_notary_signature_sample() -> NotarySignature {
    NotarySignature::sample()
}

#[uniffi::export]
pub fn new_notary_signature_sample_other() -> NotarySignature {
    NotarySignature::sample_other()
}

#[uniffi::export]
pub fn new_notary_signature(signature: Signature) -> NotarySignature {
    NotarySignature::from(signature)
}

#[uniffi::export]
pub fn notary_signature_get_signature(
    notary_signature: &NotarySignature,
) -> Signature {
    notary_signature.secret_magic
}

#[uniffi::export]
pub fn android_notarize_hash_with_private_key_bytes(
    private_key_bytes: Exactly32Bytes,
    signed_intent_hash: &SignedIntentHash,
) -> Result<NotarySignature> {
    let ed25519_private_key =
        Ed25519PrivateKey::try_from(private_key_bytes.as_ref())?;

    let private_key = PrivateKey::from(ed25519_private_key);
    let signature = private_key.notarize_hash(signed_intent_hash);

    Ok(signature)
}

#[uniffi::export]
pub fn android_sign_hash_with_private_key_bytes(
    private_key_bytes: Exactly32Bytes,
    hash: &Hash,
) -> Result<Signature> {
    let ed25519_private_key =
        Ed25519PrivateKey::try_from(private_key_bytes.as_ref())?;

    let private_key = PrivateKey::from(ed25519_private_key);
    let signature = private_key.sign(hash);

    Ok(signature)
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
