use crate::prelude::*;

/// Represents an Secp256k1 signature.
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    derive_more::Display,
    derive_more::Debug,
    derive_more::FromStr,
    uniffi::Record,
)]
#[display("{}", self.to_hex())]
#[debug("{}", self.to_hex())]
pub struct Secp256k1Signature {
    // recovery id + signature
    pub bytes: Exactly65Bytes,
}

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
            "0001598e989470d125dafac276b95bb1ba21e2ee8e0beb0547599335f83b48a0a830cd6a956a54421039cef5fb7e492ebaa315f751a2dd5b74bd9cebbda997ec12"
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
