use crate::prelude::*;
use sargon::DeviceFactorSource as InternalDeviceFactorSource;

/// A factor source representing the host device which SargonOS runs on, typically
/// an iPhone or Android device.
///
/// This is the initial factor source of
/// all new Accounts and Personas. Users authenticate signing by authorizing
/// the host to access a mnemonic stored in secure storage on
/// the device.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
     uniffi::Record,
)]
pub struct DeviceFactorSource {
    /// Unique and stable identifier of this factor source, stemming from the
    /// hash of a special child key of the HD root of the mnemonic.
    pub id: FactorSourceIDFromHash,

    /// Common properties shared between FactorSources of different kinds,
    /// describing its state, when added, and supported cryptographic parameters.
    pub common: FactorSourceCommon,

    /// Properties describing a DeviceFactorSource to help user disambiguate between it and another one.
    pub hint: DeviceFactorSourceHint,
}

impl From<InternalDeviceFactorSource> for DeviceFactorSource {
    fn from(value: InternalDeviceFactorSource) -> Self {
        Self {
            id: value.id.into(),
            common: value.common.into(),
            hint: value.hint.into(),
        }
    }
}

impl Into<InternalDeviceFactorSource> for DeviceFactorSource {
    fn into(self) -> InternalDeviceFactorSource {
        InternalDeviceFactorSource {
            id: self.id.into(),
            common: self.common.into(),
            hint: self.hint.into(),
        }
    }
}

#[uniffi::export]
pub fn new_device_factor_source_sample() -> DeviceFactorSource {
    InternalDeviceFactorSource::sample().into()
}

#[uniffi::export]
pub fn new_device_factor_source_sample_other() -> DeviceFactorSource {
    InternalDeviceFactorSource::sample_other().into()
}

#[uniffi::export]
pub fn new_device_factor_source_babylon(
    is_main: bool,
    mnemonic_with_passphrase: &MnemonicWithPassphrase,
    host_info: &HostInfo,
) -> DeviceFactorSource {
    InternalDeviceFactorSource::babylon(is_main, mnemonic_with_passphrase.into(), host_info.into()).into()
}

#[uniffi::export]
pub fn new_device_factor_source_olympia(
    mnemonic_with_passphrase: &MnemonicWithPassphrase,
    host_info: &HostInfo,
) -> DeviceFactorSource {
    InternalDeviceFactorSource::olympia(mnemonic_with_passphrase.into(), host_info.into()).into()
}

#[uniffi::export]
pub fn device_factor_source_is_main_bdfs(
    device_factor_source: &DeviceFactorSource,
) -> bool {
    device_factor_source.into_internal().is_main_bdfs()
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
