use crate::prelude::*;
use sargon::BagOfBytes as InternalBagOfBytes;

/// This is a TEMPORARY workaround until Kotlin => ByteArray equatable issue for
/// Records has been solved, see: https://github.com/mozilla/uniffi-rs/issues/1985
///
/// A bytes collection that does NOT convert into `ByteArray` in Kotlin, but
/// instead `List<Byte>`, which has a working `==`.
#[derive(Clone, PartialEq, Eq, Default, Hash)]
pub struct BagOfBytes {
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
    pub bytes: Vec<i8>,
}

delegate_display_debug_into!(BagOfBytes, InternalBagOfBytes);

impl IntoInternal<BagOfBytes, InternalBagOfBytes> for BagOfBytes {
    fn into_internal(self) -> InternalBagOfBytes {
        self.into()
    }
}

impl From<Vec<u8>> for BagOfBytes {
    fn from(value: Vec<u8>) -> Self {
        Self {
            bytes: value.into_iter().map(twos_complement_of_u8).collect(),
        }
    }
}

impl From<BagOfBytes> for Vec<u8> {
    fn from(val: BagOfBytes) -> Self {
        val.bytes.into_iter().map(twos_complement_of_i8).collect()
    }
}

impl BagOfBytes {
    pub fn to_vec(&self) -> Vec<u8> {
        self.bytes
            .iter()
            .map(|&i| twos_complement_of_i8(i))
            .collect()
    }
}

uniffi::custom_type!(BagOfBytes, Vec<i8>, {
    try_lift: |val| Ok(BagOfBytes { bytes: val }),
    lower: |obj| obj.bytes,
});

impl From<InternalBagOfBytes> for BagOfBytes {
    fn from(value: InternalBagOfBytes) -> Self {
        value.to_vec().into()
    }
}

impl From<BagOfBytes> for InternalBagOfBytes {
    fn from(val: BagOfBytes) -> Self {
        let vec: Vec<u8> = val.into();
        vec.into()
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

#[uniffi::export]
pub fn new_bag_of_bytes_from(bytes: Vec<u8>) -> BagOfBytes {
    InternalBagOfBytes::from(bytes).into()
}

#[uniffi::export]
pub fn new_bag_of_bytes_sample_aced() -> BagOfBytes {
    InternalBagOfBytes::sample_aced().into()
}

#[uniffi::export]
pub fn new_bag_of_bytes_sample_babe() -> BagOfBytes {
    InternalBagOfBytes::sample_babe().into()
}

#[uniffi::export]
pub fn new_bag_of_bytes_sample_cafe() -> BagOfBytes {
    InternalBagOfBytes::sample_cafe().into()
}

#[uniffi::export]
pub fn new_bag_of_bytes_sample_dead() -> BagOfBytes {
    InternalBagOfBytes::sample_dead().into()
}

#[uniffi::export]
pub fn new_bag_of_bytes_sample_ecad() -> BagOfBytes {
    InternalBagOfBytes::sample_ecad().into()
}

#[uniffi::export]
pub fn new_bag_of_bytes_sample_fade() -> BagOfBytes {
    InternalBagOfBytes::sample_fade().into()
}

#[uniffi::export]
pub fn bag_of_bytes_prepend_deadbeef(in_front_of: &BagOfBytes) -> BagOfBytes {
    in_front_of
        .clone()
        .into_internal()
        .prepending(vec![0xde, 0xad, 0xbe, 0xef])
        .into()
}

#[uniffi::export]
pub fn bag_of_bytes_append_deadbeef(to: &BagOfBytes) -> BagOfBytes {
    to.clone()
        .into_internal()
        .appending(vec![0xde, 0xad, 0xbe, 0xef])
        .into()
}

#[uniffi::export]
pub fn bag_of_bytes_prepend_cafe(in_front_of: &BagOfBytes) -> BagOfBytes {
    in_front_of
        .clone()
        .into_internal()
        .prepending(vec![0xca, 0xfe])
        .into()
}

#[uniffi::export]
pub fn bag_of_bytes_append_cafe(to: &BagOfBytes) -> BagOfBytes {
    to.clone()
        .into_internal()
        .appending(vec![0xca, 0xfe])
        .into()
}

decl_conversion_tests_for!(BagOfBytes);
