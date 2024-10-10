use crate::prelude::*;
use sargon::PublicKeyHash as InternalPublicKeyHash;

/// Hashes of public keys, either Ed25519PublicKey or Secp256k1PublicKey
#[derive(
    Clone,
    
    
    PartialEq,
    EnumAsInner,
    Eq,
    Hash,
    uniffi::Enum,
)]
pub enum PublicKeyHash {
    Ed25519 { value: Exactly29Bytes },
    Secp256k1 { value: Exactly29Bytes },
}

impl From<InternalPublicKeyHash> for PublicKeyHash {
    fn from(value: InternalPublicKeyHash) -> Self {
        match value {
            InternalPublicKeyHash::Ed25519 { value } => Self::Ed25519 { value },
            InternalPublicKeyHash::Secp256k1 { value } => Self::Secp256k1 { value },
        }
    }
}

impl Into<InternalPublicKeyHash> for PublicKeyHash {
    fn into(self) -> InternalPublicKeyHash {
        match self {
            PublicKeyHash::Ed25519 { value } => InternalPublicKeyHash::Ed25519 { value },
            PublicKeyHash::Secp256k1 { value } => InternalPublicKeyHash::Secp256k1 { value },
        }
    }
}

#[uniffi::export]
pub fn new_public_key_hash_of_key(public_key: PublicKey) -> PublicKeyHash {
    InternalPublicKeyHash::hash(public_key.into()).into()
}

#[uniffi::export]
pub fn new_public_key_hash_sample() -> PublicKeyHash {
    InternalPublicKeyHash::sample().into()
}

#[uniffi::export]
pub fn new_public_key_hash_sample_other() -> PublicKeyHash {
    InternalPublicKeyHash::sample_other().into()
}

