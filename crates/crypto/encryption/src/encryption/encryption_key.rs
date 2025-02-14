use crate::prelude::*;
use crypto::keys::x25519::PublicKey as X25519PublicKey;

#[derive(
    Zeroize,
    Clone,
    Copy,
    PartialEq,
    Eq,
    derive_more::Display,
    derive_more::Debug,
    derive_more::FromStr,
    Serialize,
    Deserialize,
    Hash,
)]
#[serde(transparent)]
pub struct EncryptionKey(pub Exactly32Bytes);

impl From<X25519PublicKey> for EncryptionKey {
    fn from(value: X25519PublicKey) -> EncryptionKey {
        EncryptionKey(Exactly32Bytes::from(&value.to_bytes()))
    }
}

impl EncryptionKey {
    pub fn generate() -> Self {
        Self::from(Exactly32Bytes::generate())
    }
}

impl From<Exactly32Bytes> for EncryptionKey {
    fn from(value: Exactly32Bytes) -> Self {
        Self(value)
    }
}

impl HasSampleValues for EncryptionKey {
    fn sample() -> Self {
        Self::from(Exactly32Bytes::sample())
    }
    fn sample_other() -> Self {
        Self::from(Exactly32Bytes::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = EncryptionKey;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
