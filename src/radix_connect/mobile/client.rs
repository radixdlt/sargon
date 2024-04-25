use super::relay_service::Service as RelayService;
use crate::prelude::*;

#[derive(uniffi::Object)]
pub struct RadixConnectMobile {
    relay_service: RelayService,
}

impl RadixConnectMobile {
    #[uniffi::constructor]
    pub fn new(network_antenna: Arc<dyn NetworkAntenna>) -> Self {
        Self {
            relay_service: RelayService::new(HttpClient { network_antenna }),
        }
    }
}
