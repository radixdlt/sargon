use crate::prelude::*;

/// Tries to create a PublicKey from the hex string
#[uniffi::export]
pub fn new_public_key_from_hex(hex: String) -> Result<PublicKey> {
    PublicKey::from_str(&hex)
}

/// Tries to create a PublicKey from the bytes
#[uniffi::export]
pub fn new_public_key_from_bytes(
    bag_of_bytes: BagOfBytes,
) -> Result<PublicKey> {
    PublicKey::try_from(bag_of_bytes)
}

#[uniffi::export]
pub fn public_key_to_hex(public_key: &PublicKey) -> String {
    public_key.to_hex()
}

#[uniffi::export]
pub fn public_key_to_bytes(public_key: &PublicKey) -> BagOfBytes {
    BagOfBytes::from(public_key.to_bytes())
}

#[uniffi::export]
pub fn new_public_key_sample() -> PublicKey {
    PublicKey::sample()
}

#[uniffi::export]
pub fn new_public_key_sample_other() -> PublicKey {
    PublicKey::sample_other()
}

/// Verifies an Elliptic Curve signature over either Curve25519 or Secp256k1
#[uniffi::export]
pub fn public_key_is_valid(
    public_key: &PublicKey,
    signature: Signature,
    for_hash: Hash,
) -> bool {
    public_key.is_valid(signature, &for_hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PublicKey;

    #[test]
    fn new_from_hex() {
        assert_eq!(
            new_public_key_from_hex("ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf".to_owned()).unwrap(),
            Ed25519PublicKey::sample().into()
        );
    }

    #[test]
    fn new_from_bag_of_bytes() {
        let bag_of_bytes: BagOfBytes =
            "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
                .parse()
                .unwrap();
        assert_eq!(
            new_public_key_from_bytes(bag_of_bytes).unwrap(),
            SUT::Ed25519(Ed25519PublicKey::sample())
        )
    }

    #[test]
    fn to_hex() {
        assert_eq!(
            public_key_to_hex(&SUT::sample()),
            "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
        )
    }

    #[test]
    fn to_bytes() {
        assert_eq!(
            hex_encode(public_key_to_bytes(&SUT::sample())),
            "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
        )
    }

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_public_key_sample(),
                new_public_key_sample_other(),
                // duplicates should get removed
                new_public_key_sample(),
                new_public_key_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn invalid_signature_does_not_validate() {
        assert!(!public_key_is_valid(
            &SUT::sample(),
            Signature::sample(),
            Hash::sample()
        ));
    }
}
