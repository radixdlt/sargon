use std::ops::{Deref, Neg};

use crate::prelude::*;
use radix_engine_common::crypto::{Hash, IsHash};

/// This is a TEMPORARY workaround until Kotlin => ByteArray equatable issue for
/// Records has been solved, see: https://github.com/mozilla/uniffi-rs/issues/1980
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
    ///
    bytes: Vec<u8>,
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

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(val
            .into_iter()
            // Two's complement
            .map(|i| (i.neg() as u8).wrapping_neg())
            .collect_vec()
            .into())
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.to_vec()
            .into_iter()
            // Two's complement
            .map(|u| (u.wrapping_neg() as i8).neg())
            .collect_vec()
    }
}

#[uniffi::export]
pub fn new_bag_of_bytes_from(bytes: Vec<u8>) -> BagOfBytes {
    bytes.into()
}

#[uniffi::export]
pub fn new_bag_of_bytes_placeholder_aced() -> BagOfBytes {
    BagOfBytes::placeholder_aced()
}
#[uniffi::export]
pub fn new_bag_of_bytes_placeholder_babe() -> BagOfBytes {
    BagOfBytes::placeholder_babe()
}
#[uniffi::export]
pub fn new_bag_of_bytes_placeholder_cafe() -> BagOfBytes {
    BagOfBytes::placeholder_cafe()
}
#[uniffi::export]
pub fn new_bag_of_bytes_placeholder_dead() -> BagOfBytes {
    BagOfBytes::placeholder_dead()
}
#[uniffi::export]
pub fn new_bag_of_bytes_placeholder_ecad() -> BagOfBytes {
    BagOfBytes::placeholder_ecad()
}
#[uniffi::export]
pub fn new_bag_of_bytes_placeholder_fade() -> BagOfBytes {
    BagOfBytes::placeholder_fade()
}
#[uniffi::export]
pub fn bag_of_bytes_prepend_deadbeef(in_front_of: &BagOfBytes) -> BagOfBytes {
    in_front_of.prepending(vec![0xde, 0xad, 0xbe, 0xef])
}
#[uniffi::export]
pub fn bag_of_bytes_append_deadbeef(to: &BagOfBytes) -> BagOfBytes {
    to.appending(vec![0xde, 0xad, 0xbe, 0xef])
}
#[uniffi::export]
pub fn bag_of_bytes_prepend_cafe(in_front_of: &BagOfBytes) -> BagOfBytes {
    in_front_of.prepending(vec![0xca, 0xfe])
}
#[uniffi::export]
pub fn bag_of_bytes_append_cafe(to: &BagOfBytes) -> BagOfBytes {
    to.appending(vec![0xca, 0xfe])
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
        value.into_bytes().as_slice().into()
    }
}

impl From<Vec<u8>> for BagOfBytes {
    fn from(value: Vec<u8>) -> Self {
        Self { bytes: value }
    }
}
impl From<&[u8; 32]> for BagOfBytes {
    fn from(value: &[u8; 32]) -> Self {
        Self {
            bytes: value.to_vec(),
        }
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
            .map_err(|_| CommonError::StringNotHex(s.to_owned()))
            .map(|v| v.into())
    }
}

impl HasPlaceholder for BagOfBytes {
    /// `dead...` of length 32 bytes
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::placeholder_dead()
    }

    /// A placeholder used to facilitate unit tests.
    fn placeholder_other() -> Self {
        Self::placeholder_fade()
    }
}

impl From<Hex32Bytes> for BagOfBytes {
    fn from(value: Hex32Bytes) -> Self {
        value.to_vec().into()
    }
}

impl BagOfBytes {
    /// `aced...``
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_aced() -> Self {
        Hex32Bytes::placeholder_aced().into()
    }

    /// `babe...``
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_babe() -> Self {
        Hex32Bytes::placeholder_babe().into()
    }

    /// `cafe...``
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_cafe() -> Self {
        Hex32Bytes::placeholder_cafe().into()
    }

    /// `dead...``
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_dead() -> Self {
        Hex32Bytes::placeholder_dead().into()
    }

    /// `ecad...``
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_ecad() -> Self {
        Hex32Bytes::placeholder_ecad().into()
    }

    /// `fade...``
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_fade() -> Self {
        Hex32Bytes::placeholder_fade().into()
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

#[cfg(test)]
mod tests {

    use std::ops::Neg;

    use crate::prelude::*;

    #[test]
    fn test_twos_complement() {
        // u8 -> i8
        let x: u8 = 129;
        let y: i8 = (x.wrapping_neg() as i8).neg();
        assert_eq!(y, -127);
        // and back
        let z: u8 = (y.neg() as u8).wrapping_neg();
        assert_eq!(x, z);
    }

    #[test]
    fn equality() {
        assert_eq!(BagOfBytes::placeholder(), BagOfBytes::placeholder());
        assert_eq!(
            BagOfBytes::placeholder_other(),
            BagOfBytes::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(BagOfBytes::placeholder(), BagOfBytes::placeholder_other());
    }

    #[test]
    fn len() {
        assert_eq!(BagOfBytes::placeholder().len(), 32);
    }

    #[test]
    fn default_is_empty() {
        assert_eq!(BagOfBytes::default(), BagOfBytes::new());
        assert_eq!(BagOfBytes::default().is_empty(), true);
    }

    #[test]
    fn is_empty() {
        assert_eq!(BagOfBytes::placeholder().is_empty(), false);
    }

    #[test]
    fn from_string_roundtrip() {
        let str =
            "0000000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(BagOfBytes::from_hex(str).unwrap().to_string(), str);
    }

    #[test]
    fn debug() {
        let str =
            "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead";
        let hex_bytes = BagOfBytes::placeholder();
        assert_eq!(format!("{:?}", hex_bytes), str);
    }

    #[test]
    fn display() {
        let str =
            "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead";
        let hex_bytes = BagOfBytes::placeholder();
        assert_eq!(format!("{}", hex_bytes), str);
    }

    #[test]
    fn to_hex() {
        let str =
            "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead";
        let hex_bytes = BagOfBytes::placeholder();
        assert_eq!(hex_bytes.to_string(), str);
    }

    #[test]
    fn json_roundtrip() {
        let model = BagOfBytes::placeholder();
        assert_json_value_eq_after_roundtrip(
            &model,
            json!("deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead"),
        );
    }

    #[test]
    fn json_roundtrip_fails_for_invalid() {
        assert_json_value_fails::<BagOfBytes>(json!("not even hex"));
    }

    #[test]
    fn from_bytes_roundtrip() {
        let bytes = &[0u8; 32];
        let data: BagOfBytes = bytes.into();
        assert_eq!(data.bytes(), bytes);
    }

    #[test]
    fn from_vec_roundtrip() {
        let vec = Vec::from([0u8; 32]);
        let sut: BagOfBytes = vec.clone().try_into().unwrap();
        assert_eq!(sut.to_vec(), vec);
    }

    #[test]
    fn invalid_str() {
        let s = "invalid str";
        assert_eq!(
            BagOfBytes::from_str(s),
            Err(CommonError::StringNotHex(s.to_owned()))
        );
    }

    #[test]
    fn random() {
        let mut set: HashSet<Vec<u8>> = HashSet::new();
        let n = 100;
        for _ in 0..n {
            let bytes = BagOfBytes::from(generate_32_bytes());
            set.insert(bytes.to_vec());
        }
        assert_eq!(set.len(), n);
    }

    #[test]
    fn prepend_bytes() {
        assert_eq!(
            BagOfBytes::from(vec![0xab, 0xba])
                .prepending(vec![0xca, 0xfe])
                .to_hex(),
            "cafeabba"
        )
    }
    #[test]
    fn append_bytes() {
        assert_eq!(
            BagOfBytes::from(vec![0xab, 0xba])
                .appending(vec![0xca, 0xfe])
                .to_hex(),
            "abbacafe"
        )
    }
}

#[cfg(test)]
mod uniffi_tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = BagOfBytes;

    #[test]
    fn new_ok() {
        let bytes = generate_bytes::<5>();
        assert_eq!(new_bag_of_bytes_from(bytes.clone()).bytes, bytes);
    }

    #[test]
    fn placeholders() {
        assert_eq!(
            SUT::placeholder_aced(),
            new_bag_of_bytes_placeholder_aced()
        );
        assert_eq!(
            SUT::placeholder_babe(),
            new_bag_of_bytes_placeholder_babe()
        );
        assert_eq!(
            SUT::placeholder_cafe(),
            new_bag_of_bytes_placeholder_cafe()
        );
        assert_eq!(
            SUT::placeholder_dead(),
            new_bag_of_bytes_placeholder_dead()
        );
        assert_eq!(
            SUT::placeholder_ecad(),
            new_bag_of_bytes_placeholder_ecad()
        );
        assert_eq!(
            SUT::placeholder_fade(),
            new_bag_of_bytes_placeholder_fade()
        );
    }

    #[test]
    fn append_prepend() {
        let sut: SUT = vec![0xbe, 0xef].into();
        assert_eq!(bag_of_bytes_append_cafe(&sut).to_hex(), "beefcafe");
        assert_eq!(bag_of_bytes_append_deadbeef(&sut).to_hex(), "beefdeadbeef");
        assert_eq!(bag_of_bytes_prepend_cafe(&sut).to_hex(), "cafebeef");
        assert_eq!(
            bag_of_bytes_prepend_deadbeef(&sut).to_hex(),
            "deadbeefbeef"
        );
    }
}
