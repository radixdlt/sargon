use crate::prelude::*;

/// Hashes of public keys, either Ed25519PublicKey or Secp256k1PublicKey
#[derive(Clone, Debug, PartialEq, EnumAsInner, Eq, Hash, uniffi::Enum)]
pub enum PublicKeyHash {
    Ed25519 { value: Exactly29Bytes },
    Secp256k1 { value: Exactly29Bytes },
}

impl PublicKeyHash {
    pub fn hash(key: impl Into<PublicKey>) -> Self {
        Self::hash_scrypto(key.into())
    }

    pub fn hash_scrypto(key: impl Into<ScryptoPublicKey>) -> Self {
        ScryptoPublicKeyHash::new_from_public_key(&key.into()).into()
    }
}

impl From<ScryptoPublicKeyHash> for PublicKeyHash {
    fn from(value: ScryptoPublicKeyHash) -> Self {
        match value {
            ScryptoPublicKeyHash::Secp256k1(ScryptoSecp256k1PublicKeyHash(
                bytes,
            )) => Self::Secp256k1 {
                value: Exactly29Bytes::from_bytes(&bytes),
            },
            ScryptoPublicKeyHash::Ed25519(ScryptoEd25519PublicKeyHash(
                bytes,
            )) => Self::Ed25519 {
                value: Exactly29Bytes::from_bytes(&bytes),
            },
        }
    }
}

impl From<PublicKeyHash> for ScryptoPublicKeyHash {
    fn from(value: PublicKeyHash) -> Self {
        match value {
            PublicKeyHash::Ed25519 { value } => {
                ScryptoEd25519PublicKeyHash(value.bytes()).into()
            }
            PublicKeyHash::Secp256k1 { value } => {
                ScryptoSecp256k1PublicKeyHash(value.bytes()).into()
            }
        }
    }
}
