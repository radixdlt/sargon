use crate::prelude::*;

/// Hashes of public keys, either Ed25519PublicKey or Secp256k1PublicKey
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    PartialOrd,
    Ord,
    EnumAsInner,
    Eq,
    Hash,
)]
pub enum PublicKeyHash {
    Ed25519 { value: Exactly29Bytes },
    Secp256k1 { value: Exactly29Bytes },
}

impl PublicKeyHash {
    pub fn hash(key: impl Into<PublicKey>) -> Self {
        Self::hash_scrypto(key.into())
    }

    fn hash_scrypto(key: impl Into<ScryptoPublicKey>) -> Self {
        ScryptoPublicKeyHash::new_from_public_key(&key.into()).into()
    }
}

impl From<ScryptoPublicKeyHash> for PublicKeyHash {
    fn from(value: ScryptoPublicKeyHash) -> Self {
        match value {
            ScryptoPublicKeyHash::Secp256k1(ScryptoSecp256k1PublicKeyHash(
                bytes,
            )) => Self::Secp256k1 {
                value: Exactly29Bytes::from(&bytes),
            },
            ScryptoPublicKeyHash::Ed25519(ScryptoEd25519PublicKeyHash(
                bytes,
            )) => Self::Ed25519 {
                value: Exactly29Bytes::from(&bytes),
            },
        }
    }
}

impl From<PublicKeyHash> for ScryptoPublicKeyHash {
    fn from(value: PublicKeyHash) -> Self {
        match value {
            PublicKeyHash::Ed25519 { value } => {
                ScryptoEd25519PublicKeyHash(*value.bytes()).into()
            }
            PublicKeyHash::Secp256k1 { value } => {
                ScryptoSecp256k1PublicKeyHash(*value.bytes()).into()
            }
        }
    }
}

impl HasSampleValues for PublicKeyHash {
    fn sample() -> Self {
        Self::hash(Ed25519PublicKey::sample())
    }

    fn sample_other() -> Self {
        Self::hash(Secp256k1PublicKey::sample())
    }
}
