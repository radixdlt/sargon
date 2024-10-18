use crate::prelude::*;
use sargon::HostInfoDriver as InternalHostInfoDriver;
use sargon::HostOS as InternalHostOS;

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait HostInfoDriver: Send + Sync + std::fmt::Debug {
    /// The **current** device's operating system, e.g. "iOS 17.4.1"
    async fn host_os(&self) -> HostOS;

    /// The name of the host device (iPhone/Android), e.g. "My Red iPhone"
    async fn host_device_name(&self) -> String;

    /// The **current** version of the host app, for example the Radix iOS Wallet version - e.g. "1.6.1".
    async fn host_app_version(&self) -> String;

    /// The model of the host device (iPhone/Android), .e.g. "iPhone SE 2nd Gen"
    async fn host_device_model(&self) -> String;
}

#[derive(Debug)]
pub struct HostInfoDriverAdapter {
    pub wrapped: Arc<dyn HostInfoDriver>,
}

#[async_trait::async_trait]
impl InternalHostInfoDriver for HostInfoDriverAdapter {
    async fn host_os(&self) -> InternalHostOS {
        self.wrapped.host_os().await.into()
    }

    async fn host_device_name(&self) -> String {
        self.wrapped.host_device_name().await
    }

    async fn host_app_version(&self) -> String {
        self.wrapped.host_app_version().await
    }

    async fn host_device_model(&self) -> String {
        self.wrapped.host_device_model().await
    }
}
