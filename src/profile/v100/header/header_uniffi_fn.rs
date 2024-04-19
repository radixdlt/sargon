use crate::prelude::*;

json_data_convertible!(Header);

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

#[cfg(test)]
mod uniffi_test {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Header;

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
