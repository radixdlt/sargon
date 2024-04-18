use crate::prelude::*;

#[uniffi::export]
pub fn new_device_info_sample() -> DeviceInfo {
    DeviceInfo::sample()
}

#[uniffi::export]
pub fn new_device_info_sample_other() -> DeviceInfo {
    DeviceInfo::sample_other()
}

/// Instantiates a new `DeviceInfo` with "iPhone" as description, and
/// generates a new `id` and will use the current `date` for creation date.
#[uniffi::export]
pub fn new_device_info_iphone() -> DeviceInfo {
    DeviceInfo::new_iphone()
}

#[uniffi::export]
pub fn new_device_info_from_json_string(
    json_string: String,
) -> Result<DeviceInfo> {
    DeviceInfo::new_from_json_bytes(json_string)
}

#[uniffi::export]
pub fn device_info_to_json_string(device_info: &DeviceInfo) -> String {
    device_info.to_json_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DeviceInfo;

    #[test]
    fn json_string_roundtrip() {
        let sut = SUT::sample();
        let json_str = device_info_to_json_string(&sut);
        assert_eq!(sut, new_device_info_from_json_string(json_str).unwrap());
    }

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_device_info_sample(),
                new_device_info_sample_other(),
                // duplicates should get removed
                new_device_info_sample(),
                new_device_info_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_new_device_info_iphone() {
        assert_eq!(new_device_info_iphone().description, "iPhone".to_owned())
    }
}
