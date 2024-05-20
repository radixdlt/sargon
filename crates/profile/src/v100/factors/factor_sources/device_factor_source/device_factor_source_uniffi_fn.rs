use crate::prelude::*;

#[uniffi::export]
pub fn new_device_factor_source_sample() -> DeviceFactorSource {
    DeviceFactorSource::sample()
}

#[uniffi::export]
pub fn new_device_factor_source_sample_other() -> DeviceFactorSource {
    DeviceFactorSource::sample_other()
}

#[uniffi::export]
pub fn new_device_factor_source_babylon(
    is_main: bool,
    mnemonic_with_passphrase: &MnemonicWithPassphrase,
    device_info: &DeviceInfo,
) -> DeviceFactorSource {
    DeviceFactorSource::babylon(is_main, mnemonic_with_passphrase, device_info)
}

#[uniffi::export]
pub fn new_device_factor_source_olympia(
    mnemonic_with_passphrase: &MnemonicWithPassphrase,
    device_info: &DeviceInfo,
) -> DeviceFactorSource {
    DeviceFactorSource::olympia(mnemonic_with_passphrase, device_info)
}

#[uniffi::export]
pub fn device_factor_source_is_main_bdfs(
    device_factor_source: &DeviceFactorSource,
) -> bool {
    device_factor_source.is_main_bdfs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DeviceFactorSource;

    #[test]
    fn test_new_olympia() {
        let olympia = new_device_factor_source_olympia(
            &MnemonicWithPassphrase::sample(),
            &DeviceInfo::sample(),
        );

        assert!(factor_source_supports_olympia(&olympia.clone().into()));
        assert!(!factor_source_supports_babylon(&olympia.into()));
    }

    #[test]
    fn test_new_babylon() {
        let babylon = new_device_factor_source_babylon(
            true,
            &MnemonicWithPassphrase::sample(),
            &DeviceInfo::sample(),
        );

        assert!(factor_source_supports_babylon(&babylon.clone().into()));
        assert!(!factor_source_supports_olympia(&babylon.into()));
    }

    #[test]
    fn test_new_babylon_not_main() {
        let babylon = new_device_factor_source_babylon(
            false,
            &MnemonicWithPassphrase::sample(),
            &DeviceInfo::sample(),
        );

        assert!(!device_factor_source_is_main_bdfs(&babylon));
    }

    #[test]
    fn test_new_babylon_is_main() {
        let babylon = new_device_factor_source_babylon(
            true,
            &MnemonicWithPassphrase::sample(),
            &DeviceInfo::sample(),
        );

        assert!(device_factor_source_is_main_bdfs(&babylon));
    }

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
