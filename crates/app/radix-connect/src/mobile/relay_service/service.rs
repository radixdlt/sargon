use drivers::{NetworkRequest, NetworkingDriver};
use encryption::{EncryptionScheme, VersionedEncryption};
use http_client::HttpClient;

use super::super::session::*;
use super::success_response::SuccessResponse;
use crate::prelude::*;

#[async_trait::async_trait]
pub trait WalletInteractionTransport: Send + Sync {
    async fn send_wallet_interaction_response(
        &self,
        session: Session,
        response: WalletToDappInteractionResponse,
    ) -> Result<()>;
}

/// The service that interacts with the Radix Connect Relay.
/// API docs at https://github.com/radixdlt/radix-connect-relay?tab=readme-ov-file#api-v1.
/// For now this implements only the wallet interaction response endpoint.
pub struct Service {
    pub http_client: HttpClient,
    pub encryption_scheme: EncryptionScheme,
    pub relay_service_url_resolver: Arc<dyn Fn() -> Result<Url> + Send + Sync>,
}

impl Service {
    pub fn new(
        http_client: HttpClient,
        relay_service_url_resolver: Arc<dyn Fn() -> Result<Url> + Send + Sync>,
    ) -> Self {
        Self {
            http_client,
            encryption_scheme: EncryptionScheme::default(),
            relay_service_url_resolver,
        }
    }

    pub fn new_with_networking_driver(
        networking_driver: Arc<dyn NetworkingDriver>,
        relay_service_url_resolver: Arc<dyn Fn() -> Result<Url> + Send + Sync>,
    ) -> Self {
        Self::new(
            HttpClient::new(networking_driver),
            relay_service_url_resolver,
        )
    }

    pub fn new_with_relay_service_url(
        http_client: HttpClient,
        relay_service_url: Url,
    ) -> Self {
        Self::new(http_client, Arc::new(move || Ok(relay_service_url.clone())))
    }

    pub fn new_with_networking_driver_and_relay_service_url(
        networking_driver: Arc<dyn NetworkingDriver>,
        relay_service_url: Url,
    ) -> Self {
        Self::new_with_relay_service_url(
            HttpClient::new(networking_driver),
            relay_service_url,
        )
    }
}

pub trait NetworkRequestsForRadixConnect {
    fn radix_connect_relay_request(relay_service_url: Url) -> NetworkRequest;
    fn radix_connect_success_response(
        relay_service_url: Url,
        response: SuccessResponse,
    ) -> Result<NetworkRequest> {
        Self::radix_connect_relay_request(relay_service_url)
            .with_serializing_body(response)
    }
}

impl NetworkRequestsForRadixConnect for NetworkRequest {
    fn radix_connect_relay_request(relay_service_url: Url) -> Self {
        NetworkRequest::new_post(relay_service_url)
    }
}

#[async_trait::async_trait]
impl WalletInteractionTransport for Service {
    async fn send_wallet_interaction_response(
        &self,
        session: Session,
        response: WalletToDappInteractionResponse,
    ) -> Result<()> {
        let serialized_response = response.serialize_to_bytes()?;

        let mut encryption_key = session.encryption_key;
        let encrypted_response = self
            .encryption_scheme
            .encrypt(&serialized_response, &mut encryption_key);

        let hex = hex_encode(encrypted_response);

        let success_response = SuccessResponse::new(
            session.id,
            session.wallet_public_key,
            hex.parse()?,
        );
        let relay_service_url = (self.relay_service_url_resolver)()?;
        let request = NetworkRequest::radix_connect_success_response(
            relay_service_url,
            success_response,
        )?;
        self.http_client.execute_network_request(request).await?;
        Ok(())
    }
}

#[cfg(test)]
impl Service {
    fn new_always_failing() -> Self {
        Self::new_with_networking_driver_and_relay_service_url(
            Arc::new(MockNetworkingDriver::new_always_failing()),
            Url::parse("https://radix-connect-relay.radixdlt.com/api/v1")
                .unwrap(),
        )
    }

    fn new_succeeding_http_client(request: Vec<u8>) -> Self {
        Self::new_with_networking_driver_and_relay_service_url(
            Arc::new(MockNetworkingDriver::new(200, request)),
            Url::parse("https://radix-connect-relay.radixdlt.com/api/v1")
                .unwrap(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt::time::timeout;
    use drivers::NetworkMethod;
    use encryption::{EncryptionScheme, VersionedEncryption};
    use std::time::Duration;
    const MAX: Duration = Duration::from_millis(10);

    #[actix_rt::test]
    async fn test_send_wallet_interaction_response_failure() {
        let service = Service::new_always_failing();

        let session = Session::sample();
        let req = service.send_wallet_interaction_response(
            session,
            WalletToDappInteractionResponse::sample(),
        );

        let result = timeout(MAX, req).await.unwrap();
        assert!(result.is_err());
    }

    #[actix_rt::test]
    async fn test_send_wallet_interaction_response() {
        let mock_antenna = MockNetworkingDriver::with_spy(
            200,
            BagOfBytes::new(),
            |request, _| {
                // Prepare encryption keys
                let mut encryption_key = Session::sample().encryption_key;
                let mut decryption_key = encryption_key;

                // Serialize the response
                let wallet_to_dapp_interaction_response =
                    WalletToDappInteractionResponse::sample();
                let body = wallet_to_dapp_interaction_response
                    .serialize_to_bytes()
                    .unwrap();

                // Encrypt the response
                let encrypted = EncryptionScheme::default()
                    .encrypt(&body, &mut encryption_key);
                let hex = hex_encode(encrypted);
                let success_response = SuccessResponse::new(
                    SessionID::sample(),
                    KeyAgreementPublicKey::sample(),
                    hex.parse().unwrap(),
                );

                let encoded = serde_json::to_vec(&success_response).unwrap();

                // Request that is expected to be sent
                let expected_request = NetworkRequest {
                    url: Url::from_str(
                        "https://radix-connect-relay.radixdlt.com/api/v1",
                    )
                    .unwrap(),
                    method: NetworkMethod::Post,
                    body: encoded.into(),
                    headers: HashMap::new(),
                };

                pretty_assertions::assert_eq!(
                    request.url,
                    expected_request.url
                );
                pretty_assertions::assert_eq!(
                    request.method,
                    expected_request.method
                );

                let sent_request: SuccessResponse =
                    serde_json::from_slice(&expected_request.body).unwrap();
                pretty_assertions::assert_eq!(
                    sent_request.session_id,
                    success_response.session_id
                );
                pretty_assertions::assert_eq!(
                    sent_request.method,
                    success_response.method
                );

                let decrypted_payload = EncryptionScheme::default()
                    .decrypt(sent_request.data.bytes(), &mut decryption_key)
                    .unwrap();
                let decoded_payload: WalletToDappInteractionResponse =
                    serde_json::from_slice(&decrypted_payload).unwrap();

                pretty_assertions::assert_eq!(
                    decoded_payload,
                    wallet_to_dapp_interaction_response
                )
            },
        );

        let service = Service::new_with_networking_driver_and_relay_service_url(
            Arc::new(mock_antenna),
            Url::parse("https://radix-connect-relay.radixdlt.com/api/v1")
                .unwrap(),
        );
        let session = Session::sample();

        let req = service.send_wallet_interaction_response(
            session,
            WalletToDappInteractionResponse::sample(),
        );
        let _ = timeout(MAX, req).await.unwrap();
    }
}
