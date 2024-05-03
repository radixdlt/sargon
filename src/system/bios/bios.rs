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

        error!("Error: Hello from Rust!");
        warn!("Warning: Hello from Rust!");
        info!("Info: Hello from Rust!");
        debug!("Debug: Hello from Rust!");
        trace!("Trace: Hello from Rust!");

        Arc::new(Bios { drivers })
    }
}
