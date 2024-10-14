use crate::prelude::*;
use sargon::PublicKeyHash as InternalPublicKeyHash;

/// Hashes of public keys, either Ed25519PublicKey or Secp256k1PublicKey
#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
pub enum PublicKeyHash {
    Ed25519 { value: Exactly29Bytes },
    Secp256k1 { value: Exactly29Bytes },
}

#[uniffi::export]
pub fn new_public_key_hash_of_key(public_key: PublicKey) -> PublicKeyHash {
    InternalPublicKeyHash::hash(public_key.into_internal()).into()
}

#[uniffi::export]
pub fn new_public_key_hash_sample() -> PublicKeyHash {
    InternalPublicKeyHash::sample().into()
}

#[uniffi::export]
pub fn new_public_key_hash_sample_other() -> PublicKeyHash {
    InternalPublicKeyHash::sample_other().into()
}
