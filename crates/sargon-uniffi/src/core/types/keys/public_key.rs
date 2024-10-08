use crate::prelude::*;
use sargon::PublicKey as InternalPublicKey;

/// A tagged union of supported public keys on different curves, supported
/// curves are `secp256k1` and `Curve25519`
#[derive(
    Clone,
    Debug,
    Copy,
    PartialEq,
    EnumAsInner,
    Eq,
    Hash,
    derive_more::Display,
    uniffi::Enum,
)]
pub enum PublicKey {
    /// An Ed25519 public key used to verify cryptographic signatures.
    Ed25519(Ed25519PublicKey),

    /// A secp256k1 public key used to verify cryptographic signatures (ECDSA signatures).
    Secp256k1(Secp256k1PublicKey),
}

impl From<InternalPublicKey> for PublicKey {
    fn from(value: InternalPublicKey) -> Self {
        match value {
            InternalPublicKey::Ed25519(value) => Self::Ed25519(value.into()),
            InternalPublicKey::Secp256k1(value) => Self::Secp256k1(value.into()),
        }
    }
}

impl Into<InternalPublicKey> for PublicKey {
    fn into(self) -> InternalPublicKey {
        match self {
            PublicKey::Ed25519(value) => InternalPublicKey::Ed25519(value.into()),
            PublicKey::Secp256k1(value) => InternalPublicKey::Secp256k1(value.into()),
        }
    }
}

/// Tries to create a PublicKey from the hex string
#[uniffi::export]
pub fn new_public_key_from_hex(hex: String) -> Result<PublicKey> {
    InternalPublicKey::from_str(&hex)
}

/// Tries to create a PublicKey from the bytes
#[uniffi::export]
pub fn new_public_key_from_bytes(
    bag_of_bytes: BagOfBytes,
) -> Result<PublicKey> {
    InternalPublicKey::try_from(bag_of_bytes.into())
}

#[uniffi::export]
pub fn public_key_to_hex(public_key: &PublicKey) -> String {
    public_key.into::<InternalPublicKey>().to_hex()
}

#[uniffi::export]
pub fn public_key_to_bytes(public_key: &PublicKey) -> BagOfBytes {
    InternalBagOfBytes::from(public_key.into::<InternalPublicKey>().to_bytes()).into()
}

#[uniffi::export]
pub fn new_public_key_sample() -> PublicKey {
    InternalPublicKey::sample().into()
}

#[uniffi::export]
pub fn new_public_key_sample_other() -> PublicKey {
    InternalPublicKey::sample_other().into()
}

/// Verifies an Elliptic Curve signature over either Curve25519 or Secp256k1
#[uniffi::export]
pub fn public_key_is_valid_signature_for_hash(
    public_key: &PublicKey,
    signature: Signature,
    hash: Hash,
) -> bool {
    public_key.into::<InternalPublicKey>().is_valid_signature_for_hash(signature.into(), &hash.into())
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
        assert!(!public_key_is_valid_signature_for_hash(
            &SUT::sample(),
            Signature::sample(),
            Hash::sample()
        ));
    }
}
