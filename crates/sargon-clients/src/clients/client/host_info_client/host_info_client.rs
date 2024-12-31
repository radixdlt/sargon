use crate::prelude::*;

#[derive(Debug)]
pub struct HostInfoClient {
    driver: Arc<dyn HostInfoDriver>,
}

impl HostInfoClient {
    pub(crate) fn new(driver: Arc<dyn HostInfoDriver>) -> Self {
        Self { driver }
    }

    pub(crate) async fn summary(&self) -> String {
        let host_model = self.driver.host_device_model().await;
        let host_os = self.driver.host_os().await;
        let host_app_version = self.driver.host_app_version().await;
        format!(
            "App v{} running in host OS: {} on device: {}",
            host_app_version, host_os, host_model
        )
    }

    pub async fn resolve_host_info(&self) -> HostInfo {
        let host_device_name = self.driver.host_device_name().await;
        let host_device_model = self.driver.host_device_model().await;
        let host_app_version = self.driver.host_app_version().await;
        let host_os = self.driver.host_os().await;

        HostInfo {
            description: DeviceInfoDescription::new(
                host_device_name,
                host_device_model,
            ),
            host_os,
            host_app_version,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt::time::timeout;
    use std::{future::Future, time::Duration};
    #[allow(clippy::upper_case_acronyms)]
    type SUT = HostInfoClient;

    #[actix_rt::test]
    async fn test_resolve_host_info() {
        let sut = SUT::new(RustHostInfoDriver::new());
        let mut info = sut.resolve_host_info().await;
        // Mutating this in order to keep tests stable
        info.host_app_version = "1.0.0".to_owned();

        pretty_assertions::assert_eq!(
            info,
            HostInfo::new(
                DeviceInfoDescription::new(
                    "Rosebud",
                    "Rust Sargon Unknown Device Model",
                ),
                HostOS::other("macos", "Apple", "14.5"),
                "1.0.0"
            )
        );
    }
}
