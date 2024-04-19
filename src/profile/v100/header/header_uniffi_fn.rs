use crate::prelude::*;

#[uniffi::export]
pub fn new_header_sample() -> Header {
    Header::sample()
}

#[uniffi::export]
pub fn new_header_sample_other() -> Header {
    Header::sample_other()
}

/// Instantiates a new `Header` with creating and last used on `DeviceInfo` with
/// "Unknown device" as description, and empty content hint
#[uniffi::export]
pub fn new_header_with_creating_device(creating_device: DeviceInfo) -> Header {
    Header::new(creating_device)
}

#[uniffi::export]
pub fn new_header_from_json_bytes(json_bytes: BagOfBytes) -> Result<Header> {
    Header::new_from_json_bytes(json_bytes)
}

#[uniffi::export]
pub fn header_to_json_bytes(header: &Header) -> BagOfBytes {
    header.to_json_bytes().into()
}

#[cfg(test)]
mod uniffi_test {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Header;

    #[test]
    fn json_bytes_roundtrip() {
        let sut = SUT::sample();
        let json_bytes = header_to_json_bytes(&sut);
        assert_eq!(sut, new_header_from_json_bytes(json_bytes).unwrap());
    }

    #[test]
    fn test_new_with_device() {
        assert_ne!(
            new_header_with_creating_device(DeviceInfo::sample()),
            SUT::sample()
        );
    }

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_header_sample(),
                new_header_sample_other(),
                // duplicates should get removed
                new_header_sample(),
                new_header_sample_other(),
            ])
            .len(),
            2
        );
    }
}
