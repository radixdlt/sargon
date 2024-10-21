use crate::prelude::*;
use sargon::Bios as InternalBios;
use sargon::Drivers as InternalDrivers;

#[derive(Debug, Clone, uniffi::Object)]
pub struct Bios {
    pub(crate) drivers: Arc<Drivers>,
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

impl Into<InternalBios> for Bios {
    fn into(self) -> InternalBios {
        let internal_drivers: InternalDrivers =
            self.drivers.as_ref().clone().into();
        InternalBios {
            drivers: Arc::new(internal_drivers),
        }
    }
}