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
        let host_os_version = self.driver.host_device_system_version().await;
        let host_app_version = self.driver.host_app_version().await;
        let host_vendor = self.driver.host_device_vendor().await;
        format!(
            "App v{} running in host OS: {} on device: {} ({})",
            host_app_version, host_os_version, host_model, host_vendor
        )
    }

    pub async fn create_device_info(&self) -> DeviceInfo {
        let host_name = self.driver.host_device_name().await;
        let host_model = self.driver.host_device_model().await;
        let host_os_version = self.driver.host_device_system_version().await;
        let host_app_version = self.driver.host_app_version().await;
        let host_vendor = self.driver.host_device_vendor().await;

        let maybe_device_id = self.driver.host_device_id().await;

        DeviceInfo::with_details(
            maybe_device_id,
            host_name,
            host_model,
            host_os_version,
            host_app_version,
            host_vendor,
        )
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
    async fn test_create_device_info() {
        let sut = SUT::new(RustHostInfoDriver::new());
        let mut info = sut.create_device_info().await;
        info.id = DeviceID::sample();
        info.date = Timestamp::sample();
        info.host_app_version = Some("0.0.0".to_string());
        pretty_assertions::assert_eq!(
            info,
            DeviceInfo::new(
                DeviceID::sample(),
                Timestamp::sample(),
                DeviceInfoDescription::new(
                    "Rosebud",
                    "Rust Sargon Unknown Device Model"
                ),
                "macos",
                "0.0.0",
                "Rust Sargon Unknown Vendor"
            )
        );
    }
}
