use std::ops::{Deref, DerefMut};

use crate::prelude::*;

/// A bytes collection that does NOT convert into `ByteArray` in Kotlin, but
/// instead `List<Byte>`, which has a working `==`.
#[derive(
    Zeroize, // Not `ZeroizeOnDrop`: we dont wanna zeroize all byte types: use `decl_secret_bytes!` for secrets.
    Clone,
    PartialEq,
    Eq,
    Default,
    PartialOrd,
    Ord,
    Hash,
    SerializeDisplay,
    DeserializeFromStr,
    derive_more::Display,
    derive_more::Debug,
)]
#[display("{}", self.to_hex())]
#[debug("{}", self.to_hex())]
pub struct BagOfBytes {
    pub(crate) bytes: Vec<u8>,
}

#[cfg(test)]
impl From<()> for BagOfBytes {
    fn from(_value: ()) -> Self {
        Self::new()
    }
}

impl AsRef<[u8]> for BagOfBytes {
    fn as_ref(&self) -> &[u8] {
        self.bytes.as_ref()
    }
}

impl AsMut<[u8]> for BagOfBytes {
    fn as_mut(&mut self) -> &mut [u8] {
        self.bytes.as_mut()
    }
}

impl Deref for BagOfBytes {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}
impl DerefMut for BagOfBytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bytes
    }
}

impl BagOfBytes {
    pub fn new() -> Self {
        Vec::new().into()
    }
    pub fn to_hex(&self) -> String {
        hex_encode(self.bytes())
    }
}

impl From<Vec<u8>> for BagOfBytes {
    fn from(value: Vec<u8>) -> Self {
        Self { bytes: value }
    }
}

impl From<&[u8]> for BagOfBytes {
    /// Instantiates a new `BagOfBytes` from the bytes.
    fn from(value: &[u8]) -> Self {
        Self {
            bytes: value.to_vec(),
        }
    }
}

impl FromStr for BagOfBytes {
    type Err = CommonError;

    /// Tries to decode the string `s` into a `BagOfBytes`. Will fail
    /// if the string is not valid hex or if the decoded bytes does
    /// not have length 32.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        hex_decode(s)
            .map_err(|_| CommonError::StringNotHex {
                bad_value: s.to_owned(),
            })
            .map(|v| v.into())
    }
}

impl HasSampleValues for BagOfBytes {
    /// `dead...` of length 32 bytes
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::sample_dead()
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::sample_fade()
    }
}

impl From<Exactly32Bytes> for BagOfBytes {
    fn from(value: Exactly32Bytes) -> Self {
        value.to_vec().into()
    }
}

impl BagOfBytes {
    /// `aced...``
    /// A sample used to facilitate unit tests.
    pub fn sample_aced() -> Self {
        Exactly32Bytes::sample_aced().into()
    }

    /// `babe...``
    /// A sample used to facilitate unit tests.
    pub fn sample_babe() -> Self {
        Exactly32Bytes::sample_babe().into()
    }

    /// `cafe...``
    /// A sample used to facilitate unit tests.
    pub fn sample_cafe() -> Self {
        Exactly32Bytes::sample_cafe().into()
    }

    /// `dead...``
    /// A sample used to facilitate unit tests.
    pub fn sample_dead() -> Self {
        Exactly32Bytes::sample_dead().into()
    }

    /// `ecad...``
    /// A sample used to facilitate unit tests.
    pub fn sample_ecad() -> Self {
        Exactly32Bytes::sample_ecad().into()
    }

    /// `fade...``
    /// A sample used to facilitate unit tests.
    pub fn sample_fade() -> Self {
        Exactly32Bytes::sample_fade().into()
    }
}

impl BagOfBytes {
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Returns a clone of the inner bytes as a `Vec`.
    pub fn to_vec(&self) -> Vec<u8> {
        Vec::from(self.bytes())
    }

    /// Returns a references to the inner array slice.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// For testing purposes
    pub fn prepending(&self, prefix_bytes: Vec<u8>) -> Self {
        let mut bytes = prefix_bytes;
        bytes.extend(self.to_vec());
        bytes.into()
    }

    /// For testing purposes
    pub fn appending(&self, suffix_bytes: Vec<u8>) -> Self {
        let mut bytes = self.to_vec();
        bytes.extend(suffix_bytes);
        bytes.into()
    }
}

impl BagOfBytes {
    /// Tries to decode the string `s` into a `BagOfBytes`. Will fail
    /// if the string is not valid hex or if the decoded bytes does
    /// not have length 32.
    pub fn from_hex(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

impl BagOfBytes {
    pub fn random() -> Self {
        generate_32_bytes().into()
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = BagOfBytes;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn zeroize() {
        let mut sut = SUT::sample();
        sut.zeroize();
        assert_ne!(sut, SUT::sample());
    }

    #[test]
    fn len() {
        assert_eq!(SUT::sample().len(), 32);
    }

    #[test]
    fn as_ref() {
        let b: &[u8] = &[0xde, 0xad, 0xbe, 0xef];
        assert_eq!(SUT::from(b).as_ref(), b);
    }

    #[test]
    fn deref_mut() {
        let x: &[u8] = &[0xde, 0xad, 0xbe, 0xef];
        let mut sut = SUT::sample();
        *sut = x.to_vec();
        assert_eq!(*sut, x);
    }

    #[test]
    fn as_mut() {
        let x: &[u8] = &[0xde, 0xad, 0xbe, 0xef];
        let mut sut = SUT::from(x);
        sut.as_mut()[0] = 0xff;
        assert_eq!(sut.to_hex(), "ffadbeef");
    }

    #[test]
    fn default_is_empty() {
        assert_eq!(SUT::default(), SUT::new());
        assert!(SUT::default().is_empty());
    }

    #[test]
    fn is_empty() {
        assert!(!SUT::sample().is_empty());
    }

    #[test]
    fn from_string_roundtrip() {
        let str =
            "0000000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(SUT::from_hex(str).unwrap().to_string(), str);
    }

    #[test]
    fn debug() {
        let str =
            "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead";
        let hex_bytes = SUT::sample();
        assert_eq!(format!("{:?}", hex_bytes), str);
    }

    #[test]
    fn display() {
        let str =
            "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead";
        let hex_bytes = SUT::sample();
        assert_eq!(format!("{}", hex_bytes), str);
    }

    #[test]
    fn to_hex() {
        let str =
            "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead";
        let hex_bytes = SUT::sample();
        assert_eq!(hex_bytes.to_string(), str);
    }

    #[test]
    fn json_roundtrip() {
        let model = SUT::sample();
        assert_json_value_eq_after_roundtrip(
            &model,
            json!("deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead"),
        );
    }

    #[test]
    fn deref() {
        let bytes: &[u8] = &[0xde, 0xad];
        assert_eq!(*SUT::from(bytes), bytes);
    }

    #[test]
    fn json_roundtrip_fails_for_invalid() {
        assert_json_value_fails::<BagOfBytes>(json!("not even hex"));
    }

    #[test]
    fn from_roundtrip() {
        let bytes = &[0u8; 32];
        let data: BagOfBytes = bytes.into();
        assert_eq!(data.bytes(), bytes);
    }

    #[test]
    fn from_vec_roundtrip() {
        let vec = Vec::from([0u8; 32]);
        let sut: BagOfBytes = vec.clone().into();
        assert_eq!(sut.to_vec(), vec);
    }

    #[test]
    fn invalid_str() {
        let s = "invalid str";
        assert_eq!(
            SUT::from_str(s),
            Err(CommonError::StringNotHex {
                bad_value: s.to_owned()
            })
        );
    }

    #[test]
    fn random() {
        let mut set: HashSet<Vec<u8>> = HashSet::new();
        let n = 100;
        for _ in 0..n {
            let bytes = SUT::from(generate_32_bytes());
            set.insert(bytes.to_vec());
        }
        assert_eq!(set.len(), n);
    }

    #[test]
    fn prepend_bytes() {
        assert_eq!(
            SUT::from(vec![0xab, 0xba])
                .prepending(vec![0xca, 0xfe])
                .to_hex(),
            "cafeabba"
        )
    }
    #[test]
    fn append_bytes() {
        assert_eq!(
            SUT::from(vec![0xab, 0xba])
                .appending(vec![0xca, 0xfe])
                .to_hex(),
            "abbacafe"
        )
    }
}
