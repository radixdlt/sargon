use crate::prelude::*;
use radix_engine::types::IsHash;
use radix_engine_common::crypto::{
    blake2b_256_hash, Hash as ScryptoHash, IsHash as ScryptoIsHash,
};

/// Represents a 32-byte hash digest.
///
/// Made UniFFI convertible via bytes (BagOfBytes).
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    std::hash::Hash,
    derive_more::Display,
    derive_more::FromStr,
)]
pub struct HashSecretMagic(ScryptoHash);

impl From<HashSecretMagic> for Exactly32Bytes {
    fn from(value: HashSecretMagic) -> Self {
        Exactly32Bytes::from_bytes(value.0.as_bytes())
    }
}

impl crate::UniffiCustomTypeConverter for HashSecretMagic {
    type Builtin = BagOfBytes;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Exactly32Bytes::try_from(val.bytes)
            .map(|e| e.bytes())
            .map(|b: [u8; 32]| HashSecretMagic(ScryptoHash::from_bytes(b)))
            .map_err(|e| e.into())
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        BagOfBytes::from(obj.0.into_bytes().as_slice())
    }
}

/// Represents a 32-byte hash digest.
///
/// Made UniFFI convertible via HashSecretMagic,
/// exposed in Swift/Kotlin as its own struct/data class, with
/// hidden secret magic.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    std::hash::Hash,
    derive_more::Display,
    derive_more::FromStr,
    uniffi::Record,
)]
pub struct Hash {
    pub(crate) secret_magic: HashSecretMagic,
}

impl From<Hash> for Exactly32Bytes {
    fn from(value: Hash) -> Self {
        value.secret_magic.into()
    }
}

impl AsRef<ScryptoHash> for Hash {
    fn as_ref(&self) -> &ScryptoHash {
        &self.secret_magic.0
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        self.secret_magic.0.as_ref()
    }
}

impl ScryptoIsHash for Hash {}

impl Hash {
    pub fn bytes(&self) -> Vec<u8> {
        self.secret_magic.0.clone().to_vec()
    }
}

impl From<ScryptoHash> for Hash {
    fn from(value: ScryptoHash) -> Self {
        Self {
            secret_magic: HashSecretMagic(value),
        }
    }
}
impl From<Hash> for ScryptoHash {
    fn from(value: Hash) -> Self {
        value.secret_magic.0
    }
}

/// Computes the hash digest of a message.
pub fn hash_of<T: AsRef<[u8]>>(data: T) -> Hash {
    blake2b_256_hash(data).into()
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Hash;

    #[test]
    fn test_hash() {
        assert_eq!(
            hash_of("Hello Radix".as_bytes()).to_string(),
            "48f1bd08444b5e713db9e14caac2faae71836786ac94d645b00679728202a935"
        );
    }
}
