use crate::prelude::*;

/// Properties describing a DeviceFactorSource to help user disambiguate between
/// it and another one.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{name} {model}")]
pub struct DeviceFactorSourceHint {
    /// "iPhone RED"
    pub name: String,

    /// "iPhone SE 2nd gen"
    pub model: String,

    /// The number of words in the mnemonic of a DeviceFactorSource, according to the BIP39
    /// standard, a multiple of 3, from 12 to 24 words.
    pub mnemonic_word_count: BIP39WordCount,

    /// The **last known** version of the device's operating system, e.g. "iOS 17.4.1".
    ///
    /// It is possible that the host device has been updated to a new
    /// version than recorded here, but Sargon or host clients might
    /// just not have updated this value here.
    ///
    /// MUST be optional since this was added on 2024-05-03 and
    /// was not present in earlier version of wallet (pre 1.6.0).
    pub system_version: Option<String>,

    /// The **last known** version of the host app, for example the Radix iOS Wallet version - e.g. "1.6.1"
    ///
    /// It is possible that the host device has been updated to a new
    /// version than recorded here, but Sargon or host clients might
    /// just not have updated this value here.
    ///
    /// MUST be optional since this was added on 2024-05-03 and
    /// was not present in earlier version of wallet (pre 1.6.0).
    pub host_app_version: Option<String>,

    /// The vendor of the device host, e.g. "Apple" or "Samsung".
    ///
    /// MUST be optional since this was added on 2024-05-03 and
    /// was not present in earlier version of wallet (pre 1.6.0).
    pub host_vendor: Option<String>,
}

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
    host_info: &HostInfo,
) -> DeviceFactorSource {
    DeviceFactorSource::babylon(is_main, mnemonic_with_passphrase, host_info)
}

#[uniffi::export]
pub fn new_device_factor_source_olympia(
    mnemonic_with_passphrase: &MnemonicWithPassphrase,
    host_info: &HostInfo,
) -> DeviceFactorSource {
    DeviceFactorSource::olympia(mnemonic_with_passphrase, host_info)
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
            &HostInfo::sample(),
        );

        assert!(factor_source_supports_olympia(&olympia.clone().into()));
        assert!(!factor_source_supports_babylon(&olympia.into()));
    }

    #[test]
    fn test_new_babylon() {
        let babylon = new_device_factor_source_babylon(
            true,
            &MnemonicWithPassphrase::sample(),
            &HostInfo::sample(),
        );

        assert!(factor_source_supports_babylon(&babylon.clone().into()));
        assert!(!factor_source_supports_olympia(&babylon.into()));
    }

    #[test]
    fn test_new_babylon_not_main() {
        let babylon = new_device_factor_source_babylon(
            false,
            &MnemonicWithPassphrase::sample(),
            &HostInfo::sample(),
        );

        assert!(!device_factor_source_is_main_bdfs(&babylon));
    }

    #[test]
    fn test_new_babylon_is_main() {
        let babylon = new_device_factor_source_babylon(
            true,
            &MnemonicWithPassphrase::sample(),
            &HostInfo::sample(),
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
