use crate::prelude::*;

json_data_convertible!(DeviceInfo);

#[uniffi::export]
pub fn new_device_info_sample() -> DeviceInfo {
    DeviceInfo::sample()
}

#[uniffi::export]
pub fn new_device_info_sample_other() -> DeviceInfo {
    DeviceInfo::sample_other()
}

#[uniffi::export]
pub fn new_device_info_from_host_info(
    host_id: &HostId,
    host_info: &HostInfo,
) -> DeviceInfo {
    DeviceInfo::new_from_info(host_id, host_info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DeviceInfo;

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
    fn test_new_from_host_info() {
        let host_id = HostId::sample();
        let host_info = HostInfo::sample();

        assert_eq!(
            new_device_info_from_host_info(&host_id, &host_info),
            DeviceInfo::new_from_info(&host_id, &host_info)
        )
    }
}
