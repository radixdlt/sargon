use std::ops::{Deref, DerefMut, Neg};

use crate::prelude::*;
use sargon::BagOfBytes as InternalBagOfBytes;

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
    Hash,
    InternalConversion,
     uniffi::Record,
)]
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

impl From<Vec<u8>> for BagOfBytes {
    fn from(value: Vec<u8>) -> Self {
        Self {
            bytes: value.into_iter().map(twos_complement_of_u8).collect(),
        }
    }
}

impl From<InternalBagOfBytes> for BagOfBytes {
    fn from(value: InternalBagOfBytes) -> Self {
        value.to_vec().into()
    }
}

impl Into<InternalBagOfBytes> for BagOfBytes {
    fn into(self) -> InternalBagOfBytes {
        self
        .bytes
            .into_iter()
            .map(twos_complement_of_i8)
            .collect_vec()
            .into()
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

#[cfg(test)]
mod uniffi_tests {
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
}
