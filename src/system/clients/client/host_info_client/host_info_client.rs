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
        format!(
            "App v{} running in host OS: {} on device: {}",
            host_app_version, host_os_version, host_model
        )
    }

    pub async fn create_device_info(&self) -> DeviceInfo {
        let host_name = self.driver.host_device_name().await;
        let host_model = self.driver.host_device_model().await;
        let host_os_version = self.driver.host_device_system_version().await;
        let host_app_version = self.driver.host_app_version().await;

        DeviceInfo::with_details(
            host_name,
            host_model,
            host_os_version,
            host_app_version,
        )
    }
}
