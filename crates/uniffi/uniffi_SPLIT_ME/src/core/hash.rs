use crate::prelude::*;
use sargon::Exactly32Bytes as InternalExactly32Bytes;
use sargon::Hash as InternalHash;

/// Represents a 32-byte hash digest.
///
/// Made UniFFI convertible via HashSecretMagic,
/// exposed in Swift/Kotlin as its own struct/data class, with
/// hidden secret magic.
#[derive(Clone, PartialEq, Eq, std::hash::Hash, uniffi::Record)]
pub struct Hash {
    pub(crate) value: Exactly32Bytes,
}

impl Hash {
    pub fn into_internal(&self) -> InternalHash {
        self.clone().into()
    }
}

impl From<InternalHash> for Hash {
    fn from(value: InternalHash) -> Self {
        Self {
            value: InternalExactly32Bytes::from(value).into(),
        }
    }
}

impl From<Hash> for InternalHash {
    fn from(val: Hash) -> Self {
        InternalHash::from(val.value.into_internal())
    }
}

#[uniffi::export]
pub fn hash_get_bytes(hash: &Hash) -> Exactly32Bytes {
    hash.value.clone()
}

#[uniffi::export]
pub fn new_hash_sample() -> Hash {
    InternalHash::sample().into()
}

#[uniffi::export]
pub fn new_hash_sample_other() -> Hash {
    InternalHash::sample_other().into()
}

#[uniffi::export]
pub fn new_hash_from_bytes(bytes: Exactly32Bytes) -> Hash {
    InternalHash::from(bytes.into_internal()).into()
}

#[uniffi::export]
pub fn new_hash_from_string(string: String) -> Result<Hash> {
    InternalHash::try_from(string).into_result()
}

decl_conversion_tests_for!(Hash);
