use crate::prelude::*;
use sargon::RadixConnectMobile as InternalRadixConnectMobile;

/// The Radix Connect Mobile client that handles the interaction with dApps on mobile through deepLinks.
#[derive(uniffi::Object)]
pub struct RadixConnectMobile {
    pub(crate) wrapped: Arc<InternalRadixConnectMobile>,
}

impl RadixConnectMobile {
    pub(crate) fn from_internal(
        wrapped: Arc<InternalRadixConnectMobile>,
    ) -> Self {
        Self { wrapped }
    }

    pub(crate) fn new_with_relay_service_url_resolver(
        networking_driver: Arc<dyn NetworkingDriver>,
        session_storage: Arc<dyn RadixConnectMobileSessionStorage>,
        relay_service_url_resolver: Arc<dyn Fn() -> Result<Url> + Send + Sync>,
    ) -> Self {
        let internal_relay_service_url_resolver = Arc::new(move || {
            relay_service_url_resolver().into_internal_result()
        });

        Self {
            wrapped: Arc::new(
                InternalRadixConnectMobile::new_with_relay_service_url_resolver(
                    Arc::new(NetworkingDriverAdapter {
                        wrapped: networking_driver,
                    }),
                    Arc::new(RadixConnectMobileSessionStorageAdapter {
                        wrapped: session_storage,
                    }),
                    internal_relay_service_url_resolver,
                ),
            ),
        }
    }
}

#[uniffi::export]
impl RadixConnectMobile {
    #[uniffi::constructor]
    pub fn new(
        networking_driver: Arc<dyn NetworkingDriver>,
        session_storage: Arc<dyn RadixConnectMobileSessionStorage>,
        relay_service_url: Url,
    ) -> Self {
        Self::new_with_relay_service_url_resolver(
            networking_driver,
            session_storage,
            Arc::new(move || Ok(relay_service_url.clone())),
        )
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
        self.wrapped.handle_deep_link(url).await.into_result()
    }

    /// Send the Host's response to the dApp.
    /// This is a stateful operation as it will save the session in the secure storage if the user validated the session.
    #[uniffi::method]
    pub async fn send_dapp_interaction_response(
        &self,
        wallet_response: RadixConnectMobileWalletResponse,
    ) -> Result<()> {
        self.wrapped
            .send_dapp_interaction_response(wallet_response.into())
            .await
            .into_result()
    }
}
