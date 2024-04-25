use super::relay_service::Service as RelayService;
use crate::prelude::*;

/// The Radix Connect Mobile client.
/// This is the object that will be used by the mobile app to handle interactions sent over Radix Connect Relay.
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

impl RadixConnectMobile {
    // TBA public API that will be used by the Wallet to handle deep link interactions.
}
