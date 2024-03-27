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
}
