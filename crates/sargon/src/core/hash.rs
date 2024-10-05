use crate::prelude::*;

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
    std::hash::Hash,
    derive_more::Display,
    derive_more::FromStr,
)]
pub struct Hash(pub ScryptoHash)

impl AsRef<ScryptoHash> for Hash {
    fn as_ref(&self) -> &ScryptoHash {
        &self.9
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl From<Exactly32Bytes> for Hash {
    /// Instantiates a new `Hash` from the `Exactly32Bytes`
    fn from(value: Exactly32Bytes) -> Self {
        let scrypto = ScryptoHash(*value.bytes());
        Self::from(scrypto)
    }
}

impl From<Hash> for Exactly32Bytes {
    /// Instantiates a new `Exactly32Bytes` from the `Hash`
    fn from(value: Hash) -> Self {
        Self::from(&value.into_bytes())
    }
}

impl TryFrom<String> for Hash {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        let bytes = Exactly32Bytes::from_str(&string)?
        Ok(Self::from(bytes))
    }
}

impl ScryptoIsHash for Hash {}

impl Hash {
    pub fn bytes(&self) -> Vec<u8> {
        self.0.clone().to_vec()
    }
}

impl From<ScryptoHash> for Hash {
    fn from(value: ScryptoHash) -> Self {
        Self(value)
    }
}
impl From<Hash> for ScryptoHash {
    fn from(value: Hash) -> Self {
        value.0
    }
}

/// Computes the hash digest of a message.
pub fn hash_of<T: AsRef<[u8]>>(data: T) -> Hash {
    blake2b_256_hash(data).into()
}

impl HasSampleValues for Hash {
    fn sample() -> Self {
        // "Hello Radix".as_bytes()
        "48f1bd08444b5e713db9e14caac2faae71836786ac94d645b00679728202a935"
            .parse::<Self>()
            .unwrap()
    }

    fn sample_other() -> Self {
        // "Radix... just imagine".as_bytes()
        "679dfbbda16d3f9669da8552ab6594d2b0446d03d96c076a0ec9dc550ea41fad"
            .parse::<Self>()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {

    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Hash;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn test_from_exactly32() {
        let bytes = Exactly32Bytes::sample();
        assert_eq!(SUT::from(bytes).bytes(), bytes.to_vec())
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn test_hash() {
        assert_eq!(
            hash_of("Hello Radix".as_bytes()).to_string(),
            "48f1bd08444b5e713db9e14caac2faae71836786ac94d645b00679728202a935"
        );

        assert_eq!(
            hash_of("Radix... just imagine".as_bytes()).to_string(),
            "679dfbbda16d3f9669da8552ab6594d2b0446d03d96c076a0ec9dc550ea41fad"
        );
    }

    #[test]
    fn hash_of_hash() {
        assert_eq!(
            hash_of(SUT::sample()).to_string(),
            "0c18fa9b3e94d9b879d631e791ee0699ad2f98d914f16a35a70f6312abe4474a"
        );
    }

    #[test]
    fn to_string() {
        assert_eq!(
            SUT::sample_other().to_string(),
            "679dfbbda16d3f9669da8552ab6594d2b0446d03d96c076a0ec9dc550ea41fad"
        );
    }

    #[test]
    fn from_str() {
        assert_eq!(
            "48f1bd08444b5e713db9e14caac2faae71836786ac94d645b00679728202a935"
                .parse::<SUT>()
                .unwrap(),
            hash_of("Hello Radix".as_bytes())
        );
    }

    #[test]
    fn manual_perform_uniffi_conversion_successful() {
        let sut = SUT::sample().secret_magic;
        let builtin = BagOfBytes::from_hex(
            "48f1bd08444b5e713db9e14caac2faae71836786ac94d645b00679728202a935",
        )
        .unwrap();

        let ffi_side =
            <HashSecretMagic as crate::UniffiCustomTypeConverter>::from_custom(
                sut,
            );

        assert_eq!(ffi_side.to_hex(), builtin.to_hex());

        let from_ffi_side =
            <HashSecretMagic as crate::UniffiCustomTypeConverter>::into_custom(
                ffi_side,
            )
            .unwrap();

        assert_eq!(sut, from_ffi_side);
    }

    #[test]
    fn manual_perform_uniffi_conversion_fail() {
        assert!(
            <HashSecretMagic as crate::UniffiCustomTypeConverter>::into_custom(
                BagOfBytes::from_hex("deadbeef").unwrap(),
            )
            .is_err()
        );
    }
}
