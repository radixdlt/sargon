use crate::prelude::*;
use sargon::RadixConnectMobile as InternalRadixConnectMobile;

/// The Radix Connect Mobile client that handles the interaction with dApps on mobile through deepLinks.
#[derive(uniffi::Object)]
pub struct RadixConnectMobile {
    wrapped: Arc<InternalRadixConnectMobile>,
}

#[uniffi::export]
impl RadixConnectMobile {
    #[uniffi::constructor]
    pub fn new(
        networking_driver: Arc<dyn NetworkingDriver>,
        session_storage: Arc<dyn RadixConnectMobileSessionStorage>,
    ) -> Self {
        Self {
            wrapped: Arc::new(InternalRadixConnectMobile::new(
                Arc::new(NetworkingDriverAdapter {
                    wrapped: networking_driver,
                }),
                Arc::new(RadixConnectMobileSessionStorageAdapter {
                    wrapped: session_storage,
                }),
            )),
        }
    }
}

#[uniffi::export]
impl RadixConnectMobile {
    /// Try to parse the deep link and create a RadixConnectMobileDappRequest.
    /// This is a stateful operation as it will create an in flight session, that needs to be validated by the user.
    #[uniffi::method]
    pub async fn handle_deep_link(
        &self,
        url: String,
    ) -> Result<RadixConnectMobileSessionRequest> {
        map_result_from_internal(self.wrapped.handle_deep_link(url).await)
    }

    /// Send the Host's response to the dApp.
    /// This is a stateful operation as it will save the session in the secure storage if the user validated the session.
    #[uniffi::method]
    pub async fn send_dapp_interaction_response(
        &self,
        wallet_response: RadixConnectMobileWalletResponse,
    ) -> Result<()> {
        map_result_from_internal(
            self.wrapped
                .send_dapp_interaction_response(wallet_response.into())
                .await,
        )
    }
}
