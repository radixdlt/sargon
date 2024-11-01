use crate::prelude::*;
use sargon::Bios as InternalBios;
use sargon::Drivers as InternalDrivers;

#[derive(Debug, Clone, uniffi::Object)]
pub struct Bios {
    pub drivers: Arc<Drivers>,
}

#[uniffi::export]
impl Bios {
    #[uniffi::constructor]
    pub fn new(drivers: Arc<Drivers>) -> Arc<Self> {
        Arc::new(Bios { drivers })
    }
}

impl Bios {
    pub fn into_internal(&self) -> InternalBios {
        self.clone().into()
    }
}

impl From<Bios> for InternalBios {
    fn from(val: Bios) -> Self {
        let internal_drivers: InternalDrivers =
            val.drivers.as_ref().clone().into();
        InternalBios {
            drivers: Arc::new(internal_drivers),
        }
    }
}
