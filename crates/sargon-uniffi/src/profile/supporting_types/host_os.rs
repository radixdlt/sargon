use crate::prelude::*;
use sargon::HostOS as InternalHostOS;

/// Describes the type of the Host machine and its version. Currently, as it stands at runtime
/// the possible values will be IOS or Android. Other is in place to facilitate unit tests
/// and to make sargon host agnostic.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
pub enum HostOS {
    IOS {
        version: String,
    },
    Android {
        vendor: String,
        version: String,
    },
    Other {
        name: String,
        vendor: String,
        version: String,
    },
}

#[uniffi::export]
pub fn new_host_os_ios(version: String) -> HostOS {
    InternalHostOS::ios(version).into()
}

#[uniffi::export]
pub fn new_host_os_android(vendor: String, version: String) -> HostOS {
    InternalHostOS::android(vendor, version).into()
}

#[uniffi::export]
pub fn new_host_os_other(
    name: String,
    vendor: String,
    version: String,
) -> HostOS {
    InternalHostOS::other(name, vendor, version).into()
}

#[uniffi::export]
pub fn host_os_get_name(host_os: &HostOS) -> String {
    host_os.into_internal().name()
}

#[uniffi::export]
pub fn host_os_get_vendor(host_os: &HostOS) -> String {
    host_os.into_internal().vendor()
}

#[uniffi::export]
pub fn host_os_get_version(host_os: &HostOS) -> String {
    host_os.into_internal().version()
}

#[uniffi::export]
pub fn new_host_os_sample() -> HostOS {
    InternalHostOS::sample().into()
}

#[uniffi::export]
pub fn new_host_os_sample_other() -> HostOS {
    InternalHostOS::sample_other().into()
}
