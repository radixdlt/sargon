use crate::prelude::*;
use sargon::Hash as InternalHash;
use sargon::Exactly32Bytes as InternalExactly32Bytes;

/// Represents a 32-byte hash digest.
///
/// Made UniFFI convertible via HashSecretMagic,
/// exposed in Swift/Kotlin as its own struct/data class, with
/// hidden secret magic.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Copy,
    Eq,
    derive_more::Display,
    uniffi::Record,
)]
pub struct Hash {
    pub(crate) secret_magic: BagOfBytes,
}

impl From<InternalHash> for Hash {
    fn from(value: InternalHash) -> Self {
        Self {
            secret_magic: BagOfBytes::from(value.0.into_bytes().as_slice()),
        }
    }
}

impl TryInto<InternalHash> for Hash {
    type Error = CommonError;

    fn try_into(self) -> Result<InternalHash> {
        let bytes = Exactly32Bytes::try_from(self.secret_magic.bytes)?;
        Ok(InternalHash::from(bytes.into()))
    }
}

#[uniffi::export]
pub fn hash_get_bytes(hash: &Hash) -> Exactly32Bytes {
    InternalExactly32Bytes::from(*hash.into()).into()
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
    InternalHash::from(bytes.into()).into()
}

#[uniffi::export]
pub fn new_hash_from_string(string: String) -> Result<Hash> {
    map_result_from_internal(InternalHash::try_from(string))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Hash;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_hash_sample(),
                new_hash_sample_other(),
                // duplicates should get removed
                new_hash_sample(),
                new_hash_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_hash_get_bytes() {
        assert_eq!(
            hex_encode(hash_get_bytes(&SUT::sample())),
            "48f1bd08444b5e713db9e14caac2faae71836786ac94d645b00679728202a935"
        );
    }

    #[test]
    fn test_from_str() {
        let sut = new_hash_from_string(
            "48f1bd08444b5e713db9e14caac2faae71836786ac94d645b00679728202a935"
                .to_owned(),
        )
        .unwrap();
        assert_eq!(sut, SUT::sample())
    }

    #[test]
    fn test_from_bytes() {
        let bytes = Exactly32Bytes::from_str(
            "48f1bd08444b5e713db9e14caac2faae71836786ac94d645b00679728202a935",
        )
        .unwrap();
        let sut = new_hash_from_bytes(bytes);
        assert_eq!(sut, SUT::sample())
    }
}
