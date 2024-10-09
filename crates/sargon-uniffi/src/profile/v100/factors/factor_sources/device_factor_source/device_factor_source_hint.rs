use crate::prelude::*;
use sargon::DeviceFactorSourceHint as InternalDeviceFactorSourceHint;

/// Properties describing a DeviceFactorSource to help user disambiguate between
/// it and another one.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
     uniffi::Record,
)]
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

impl From<InternalDeviceFactorSourceHint> for DeviceFactorSourceHint {
    fn from(value: InternalDeviceFactorSourceHint) -> Self {
        Self {
            name: value.name,
            model: value.model,
            mnemonic_word_count: value.mnemonic_word_count.into(),
            system_version: value.system_version,
            host_app_version: value.host_app_version,
            host_vendor: value.host_vendor,
        }
    }
}

impl Into<InternalDeviceFactorSourceHint> for DeviceFactorSourceHint {
    fn into(self) -> InternalDeviceFactorSourceHint {
        InternalDeviceFactorSourceHint {
            name: self.name,
            model: self.model,
            mnemonic_word_count: self.mnemonic_word_count.into(),
            system_version: self.system_version,
            host_app_version: self.host_app_version,
            host_vendor: self.host_vendor,
        }
    }
}