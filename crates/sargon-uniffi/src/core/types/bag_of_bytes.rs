use std::ops::{Deref, DerefMut, Neg};

use crate::prelude::*;
use sargon::BagOfBytes as InternalBagOfBytes;

/// This is a TEMPORARY workaround until Kotlin => ByteArray equatable issue for
/// Records has been solved, see: https://github.com/mozilla/uniffi-rs/issues/1985
///
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
    derive_more::Display,
    derive_more::Debug,
)]
pub struct BagOfBytes {
    pub(crate) bytes: Vec<u8>,
}

impl From<InternalBagOfBytes> for BagOfBytes {
    fn from(value: InternalBagOfBytes) -> Self {
        value.bytes().into()
    }
}

impl Into<InternalBagOfBytes> for BagOfBytes {
    fn into(self) -> InternalBagOfBytes {
        self.bytes.into()
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

#[uniffi::export]
pub fn new_bag_of_bytes_from(bytes: Vec<u8>) -> BagOfBytes {
    bytes.into()
}

#[uniffi::export]
pub fn new_bag_of_bytes_sample_aced() -> BagOfBytes {
    BagOfBytes::sample_aced()
}

#[uniffi::export]
pub fn new_bag_of_bytes_sample_babe() -> BagOfBytes {
    BagOfBytes::sample_babe()
}

#[uniffi::export]
pub fn new_bag_of_bytes_sample_cafe() -> BagOfBytes {
    BagOfBytes::sample_cafe()
}

#[uniffi::export]
pub fn new_bag_of_bytes_sample_dead() -> BagOfBytes {
    BagOfBytes::sample_dead()
}

#[uniffi::export]
pub fn new_bag_of_bytes_sample_ecad() -> BagOfBytes {
    BagOfBytes::sample_ecad()
}

#[uniffi::export]
pub fn new_bag_of_bytes_sample_fade() -> BagOfBytes {
    BagOfBytes::sample_fade()
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

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = BagOfBytes;

    #[test]
    fn new_ok() {
        let bytes = generate_bytes::<5>();
        assert_eq!(new_bag_of_bytes_from(bytes.clone()).bytes, bytes);
    }

    #[test]
    fn sample_values() {
        assert_eq!(SUT::sample_aced(), new_bag_of_bytes_sample_aced());
        assert_eq!(SUT::sample_babe(), new_bag_of_bytes_sample_babe());
        assert_eq!(SUT::sample_cafe(), new_bag_of_bytes_sample_cafe());
        assert_eq!(SUT::sample_dead(), new_bag_of_bytes_sample_dead());
        assert_eq!(SUT::sample_ecad(), new_bag_of_bytes_sample_ecad());
        assert_eq!(SUT::sample_fade(), new_bag_of_bytes_sample_fade());
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
