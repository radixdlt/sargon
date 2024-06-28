use std::env;

use crate::prelude::*;

#[derive(Debug)]
pub struct RustHostInfoDriver;

impl RustHostInfoDriver {
    pub fn new() -> Arc<Self> {
        Arc::new(RustHostInfoDriver)
    }
}

#[async_trait::async_trait]
impl HostInfoDriver for RustHostInfoDriver {
    /// The name of the host device (iPhone/Android), e.g. "My Red iPhone"
    async fn host_device_name(&self) -> String {
        "Rosebud".to_owned()
    }

    /// The **current** version of the device's operating system, e.g. "iOS 17.4.1"
    async fn host_device_system_version(&self) -> String {
        format!("{}", env::consts::OS)
    }

    /// The **current** version of the host app, for example the Radix iOS Wallet version - e.g. "1.6.1".
    async fn host_app_version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_owned()
    }

    /// The model of the host device (iPhone/Android), .e.g. "iPhone SE 2nd Gen"
    async fn host_device_model(&self) -> String {
        "Rust Sargon Unknown Device Model".to_owned()
    }

    async fn host_device_id(&self) -> Option<DeviceID> {
        None
    }

    async fn host_device_vendor(&self) -> String {
        "Rust Sargon Unknown Vendor".to_owned()
    }
}
