use crate::prelude::*;
use sargon::DeviceFactorSource as InternalDeviceFactorSource;

/// A factor source representing the host device which SargonOS runs on, typically
/// an iPhone or Android device.
///
/// This is the initial factor source of
/// all new Accounts and Personas. Users authenticate signing by authorizing
/// the host to access a mnemonic stored in secure storage on
/// the device.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
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
    mnemonic_with_passphrase: &MnemonicWithPassphrase,
    host_info: &HostInfo,
) -> DeviceFactorSource {
    InternalDeviceFactorSource::babylon(
        &mnemonic_with_passphrase.into_internal(),
        &host_info.into_internal(),
    )
    .into()
}

#[uniffi::export]
pub fn new_device_factor_source_olympia(
    mnemonic_with_passphrase: &MnemonicWithPassphrase,
    host_info: &HostInfo,
) -> DeviceFactorSource {
    InternalDeviceFactorSource::olympia(
        &mnemonic_with_passphrase.into_internal(),
        &host_info.into_internal(),
    )
    .into()
}
