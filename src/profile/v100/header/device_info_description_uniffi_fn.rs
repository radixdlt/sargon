use crate::prelude::*;

#[uniffi::export]
pub fn new_device_info_description_sample() -> DeviceInfoDescription {
    DeviceInfoDescription::sample()
}

#[uniffi::export]
pub fn new_device_info_description_sample_other() -> DeviceInfoDescription {
    DeviceInfoDescription::sample_other()
}

#[uniffi::export]
pub fn device_info_description_to_string(
    device_info_description: &DeviceInfoDescription,
) -> String {
    device_info_description.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DeviceInfoDescription;

    #[test]
    fn test_to_string() {
        let sut = SUT::sample();

        assert_eq!(sut.to_string(), device_info_description_to_string(&sut))
    }

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_device_info_description_sample(),
                new_device_info_description_sample_other(),
                // duplicates should get removed
                new_device_info_description_sample(),
                new_device_info_description_sample_other(),
            ])
            .len(),
            2
        );
    }
}
