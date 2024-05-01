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
    private_key_bytes: Entropy32Bytes,
    signed_intent_hash: &SignedIntentHash,
) -> Result<NotarySignature> {
    let mut private_key_bytes = private_key_bytes;

    let ed25519_private_key =
        Ed25519PrivateKey::from_bytes(private_key_bytes.to_bytes())?;

    let private_key = PrivateKey::from(ed25519_private_key);
    let signature = private_key.notarize_hash(signed_intent_hash);
    private_key_bytes.zeroize();
    // private_key.zeroize() // FIXME: Zeroize once RET has added Zeroize to PrivateKeys

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
            Entropy32Bytes::sample(),
            &SignedIntentHash::sample(),
        )
        .unwrap();

        assert_eq!(
            "08c6129fa6938a31e38dfe94effdce8f1a4021e22cf62344830d83dc45f32de0e3d112794c369450e62d245a17b18835f40c639033fbb4b1f975ad0ad71dbf0a",
            sut.to_string()
        )
    }
}
