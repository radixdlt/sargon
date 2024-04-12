use crate::prelude::*;

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
        assert_eq!(*new_bag_of_bytes_from(bytes.clone()).bytes, bytes);
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
