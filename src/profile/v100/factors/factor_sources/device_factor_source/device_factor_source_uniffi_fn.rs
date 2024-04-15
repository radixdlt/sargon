use crate::prelude::*;

#[uniffi::export]
pub fn new_device_factor_source_sample() -> DeviceFactorSource {
    DeviceFactorSource::sample()
}

#[uniffi::export]
pub fn new_device_factor_source_sample_other() -> DeviceFactorSource {
    DeviceFactorSource::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DeviceFactorSource;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_device_factor_source_sample(),
                new_device_factor_source_sample_other(),
                // duplicates should get removed
                new_device_factor_source_sample(),
                new_device_factor_source_sample_other(),
            ])
            .len(),
            2
        );
    }
}
