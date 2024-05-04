use crate::prelude::*;

#[derive(Debug, uniffi::Object)]
pub struct Bios {
    pub(crate) drivers: Arc<Drivers>,
}

#[uniffi::export]
impl Bios {
    #[uniffi::constructor]
    pub fn new(drivers: Arc<Drivers>) -> Arc<Self> {
        install_logger(drivers.logging_driver.clone());
        Arc::new(Bios { drivers })
    }
}
