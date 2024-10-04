use crate::prelude::*;
use sargon::Bios as InternalBios;

#[derive(Debug, uniffi::Object)]
pub struct Bios {
    pub(crate) drivers: Arc<Drivers>,
}

#[uniffi::export]
impl Bios {
    #[uniffi::constructor]
    pub fn new(drivers: Arc<Drivers>) -> Arc<Self> {
        install_logger(drivers.logging.clone());
        Arc::new(Bios { drivers })
    }
}

impl Into<InternalBios> for Bios {
    fn into(self) -> InternalBios {
        InternalBios {
            drivers: self.drivers.into(),
        }
    }
}
