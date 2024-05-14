use crate::prelude::*;

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait HostInfoDriver: Send + Sync + std::fmt::Debug {
    /// The name of the host device (iPhone/Android), e.g. "My Red iPhone"
    async fn host_device_name(&self) -> String;

    /// The **current** version of the device's operating system, e.g. "iOS 17.4.1"
    async fn host_device_system_version(&self) -> String;

    /// The **current** version of the host app, for example the Radix iOS Wallet version - e.g. "1.6.1".
    async fn host_app_version(&self) -> String;

    /// The model of the host device (iPhone/Android), .e.g. "iPhone SE 2nd Gen"
    async fn host_device_model(&self) -> String;

    /// A **stable** id that unique identified the host device (iPhone/Android), this
    /// MUST be stable and SHOULD NOT be generated by the host device, if the host OS
    /// does not support this (iOS does not), then return `None`.
    ///
    /// Android seems to support it if `READ_PRIVILEGED_PHONE_STATE` permission is
    /// obtained https://source.android.com/docs/core/connect/device-identifiers
    async fn host_device_id(&self) -> Option<DeviceID>;
}
