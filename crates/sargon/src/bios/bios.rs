use crate::prelude::*;

#[derive(Debug, uniffi::Object)]
pub struct Bios {
    pub drivers: Arc<Drivers>,
}

#[uniffi::export]
impl Bios {
    #[uniffi::constructor]
    pub fn new(drivers: Arc<Drivers>) -> Arc<Self> {
        install_logger(drivers.logging.clone());
        Arc::new(Bios { drivers })
    }
}
