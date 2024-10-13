use crate::prelude::*;
use sargon::HostInfo as InternalHostInfo;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Record)]
pub struct HostInfo {
    /// A short description of the device. The host should
    /// read the device model and a given name from the device
    /// if they are able to.
    pub description: DeviceInfoDescription,

    /// The **current** os and version of the device's operating system, e.g. "iOS 17.4.1".
    pub host_os: HostOS,

    /// The **current** version of the host app, for example the Radix iOS Wallet version - e.g. "1.6.1"
    pub host_app_version: String,
}

#[uniffi::export]
pub fn new_host_info_sample() -> HostInfo {
    InternalHostInfo::sample().into()
}

#[uniffi::export]
pub fn new_host_info_sample_other() -> HostInfo {
    InternalHostInfo::sample_other().into()
}
