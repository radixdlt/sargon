use crate::prelude::*;

pub struct Bios {
    pub drivers: Arc<Drivers>,
}

impl Bios {
    pub fn new(drivers: Arc<Drivers>) -> Arc<Self> {
        install_logger(drivers.logging.clone());
        Arc::new(Bios { drivers })
    }
}
