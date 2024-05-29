use crate::prelude::*;

#[uniffi::export]
pub fn new_off_device_mnemonic_factor_source_sample(
) -> OffDeviceMnemonicFactorSource {
    OffDeviceMnemonicFactorSource::sample()
}

#[uniffi::export]
pub fn new_off_device_mnemonic_factor_source_sample_other(
) -> OffDeviceMnemonicFactorSource {
    OffDeviceMnemonicFactorSource::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = OffDeviceMnemonicFactorSource;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_off_device_mnemonic_factor_source_sample(),
                new_off_device_mnemonic_factor_source_sample_other(),
                // duplicates should get removed
                new_off_device_mnemonic_factor_source_sample(),
                new_off_device_mnemonic_factor_source_sample_other(),
            ])
            .len(),
            2
        );
    }
}
