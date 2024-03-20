use std::ops::{Deref, Neg};

use crate::prelude::*;

/// This is a TEMPORARY workaround until Kotlin => ByteArray equatable issue for
/// Records has been solved, see: https://github.com/mozilla/uniffi-rs/issues/1985
///
/// A bytes collection that does NOT convert into `ByteArray` in Kotlin, but
/// instead `List<Byte>`, which has a working `==`.
#[derive(
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

impl AsRef<[u8]> for BagOfBytes {
    fn as_ref(&self) -> &[u8] {
        self.bytes.as_ref()
    }
}

impl Deref for BagOfBytes {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

/// Expose `BagOfBytes` to Uniffi as `sequence<i8>`, unfortunately we cannot
/// use `sequence<u8>` because it results in:
///
/// /uniffi-rs-6f89edd2a1ffa4bd/fb8dd5c/uniffi_bindgen/src/interface/universe.rs:50:17:
/// assertion `left == right` failed
/// left: Custom { module_path: "profile", name: "BagOfBytes", builtin: Bytes }
/// right: Custom { module_path: "profile", name: "BagOfBytes", builtin: Sequence { inner_type: UInt8 } }
///
/// So HACK HACK HACK we use `sequence<i8>` (`Vec<i8>`) instead as an intermediary `Builtin`.
///
/// However, in `uniffi.toml` we provide `from_custom`` / `into_custom`` for Kotlin and Swift
/// which using two's complement maps back Vec<i8> -> Vec<u8>, meaning Kotlin and Swift actually
/// never see the `i8`, and only works with u8.
///
/// So we translate:
/// Kotlin: `Rust[BagOfBytes <:2's comp.:> Vec<i8>] <:2's comp:> [Kotlin]List<UByte>`
/// Swift:  `Rust[BagOfBytes <:2's comp.:> Vec<i8>] <:2's comp:> [Swift]Foundation.Data`
///
impl crate::UniffiCustomTypeConverter for BagOfBytes {
    type Builtin = Vec<i8>;

    #[cfg(not(tarpaulin_include))] // false negative, tested in bindgen tests
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(val
            .into_iter()
            .map(twos_complement_of_i8)
            .collect_vec()
            .into())
    }

    #[cfg(not(tarpaulin_include))] // false negative, tested in bindgen tests
    fn from_custom(obj: Self) -> Self::Builtin {
        obj.to_vec()
            .into_iter()
            .map(twos_complement_of_u8)
            .collect_vec()
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

impl From<Hash> for BagOfBytes {
    /// Instantiates a new `BagOfBytes` from the `Hash` (32 bytes).
    fn from(value: Hash) -> Self {
        value.bytes().as_slice().into()
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

fn twos_complement_of_u8(u: u8) -> i8 {
    // Yes, it is this easy, Rust does all the heavy lifting
    u as i8
}

fn twos_complement_of_i8(i: i8) -> u8 {
    // Yes, it is this easy, Rust does all the heavy lifting
    i as u8
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = BagOfBytes;

    #[test]
    fn test_twos_complement() {
        // basics
        assert_eq!(twos_complement_of_u8(130), -126);

        let uiu = |u: u8| twos_complement_of_i8(twos_complement_of_u8(u));
        let t_uiu = |u: u8| assert_eq!(uiu(u), u);
        t_uiu(0);
        t_uiu(1);
        t_uiu(2);
        t_uiu(126);
        t_uiu(127);
        t_uiu(128);
        t_uiu(129);
        t_uiu(130);
        t_uiu(254);
        t_uiu(255);

        let iui = |i: i8| twos_complement_of_u8(twos_complement_of_i8(i));
        let t_iui = |i: i8| assert_eq!(iui(i), i);

        t_iui(-128);
        t_iui(-127);
        t_iui(-2);
        t_iui(-1);
        t_iui(0);
        t_iui(1);
        t_iui(2);
        t_iui(126);
        t_iui(127);
    }

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
    fn len() {
        assert_eq!(SUT::sample().len(), 32);
    }

    #[test]
    fn as_ref() {
        let b: &[u8] = &[0xde, 0xad, 0xbe, 0xef];
        assert_eq!(SUT::from(b).as_ref(), b);
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
    fn from_hash() {
        let digest = hash_of(vec![0xde, 0xad]);
        assert_eq!(SUT::from(digest).to_vec(), digest.bytes());
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
