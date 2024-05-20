use crate::prelude::*;

#[uniffi::export]
pub fn new_secp256k1_signature_sample() -> Secp256k1Signature {
    Secp256k1Signature::sample()
}

#[uniffi::export]
pub fn new_secp256k1_signature_sample_other() -> Secp256k1Signature {
    Secp256k1Signature::sample_other()
}

#[uniffi::export]
pub fn new_secp256k1_signature_from_exactly_65_bytes(
    bytes: Exactly65Bytes,
) -> Secp256k1Signature {
    Secp256k1Signature::from(bytes)
}

#[uniffi::export]
pub fn new_secp256k1_signature_from_bytes(
    bytes: BagOfBytes,
) -> Result<Secp256k1Signature> {
    Secp256k1Signature::try_from(bytes)
}

#[uniffi::export]
pub fn secp256k1_signature_to_string(signature: &Secp256k1Signature) -> String {
    signature.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Secp256k1Signature;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_secp256k1_signature_sample(),
                new_secp256k1_signature_sample_other(),
                // duplicates should get removed
                new_secp256k1_signature_sample(),
                new_secp256k1_signature_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            secp256k1_signature_to_string(&SUT::sample()),
            "018ad795353658a0cd1b513c4414cbafd0f990d329522977f8885a27876976a7d41ed8a81c1ac34551819627689cf940c4e27cacab217f00a0a899123c021ff6ef"
        )
    }

    #[test]
    fn test_from_exactly_65_bytes() {
        let sut = SUT::sample();
        assert_eq!(
            new_secp256k1_signature_from_exactly_65_bytes(sut.bytes),
            sut
        )
    }

    #[test]
    fn test_from_bag_of_bytes() {
        let sut = SUT::sample();
        assert_eq!(
            new_secp256k1_signature_from_bytes(BagOfBytes::from(
                sut.to_bytes()
            ))
            .unwrap(),
            sut
        )
    }
}
