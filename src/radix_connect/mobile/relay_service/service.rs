use hex::ToHex;

use super::super::session::*;
use crate::prelude::*;
use crate::radix_connect::mobile::relay_service::SuccessResponse;

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
}

impl Service {
    pub fn new(http_client: HttpClient) -> Self {
        Self {
            http_client,
            encryption_scheme: EncryptionScheme::default(),
        }
    }

    pub fn new_with_network_antenna(
        network_antenna: Arc<dyn NetworkAntenna>,
    ) -> Self {
        Self::new(HttpClient::new(network_antenna))
    }
}

const SERVICE_PATH: &str = "https://radix-connect-relay.radixdlt.com/api/v1";

impl NetworkRequest {
    fn radix_connect_relay_request() -> Self {
        NetworkRequest::new_post(Url::from_str(SERVICE_PATH).unwrap())
    }

    fn radix_connect_success_response(
        response: SuccessResponse,
    ) -> Result<Self> {
        Self::radix_connect_relay_request().with_serializing_body(response)
    }
}

#[async_trait::async_trait]
impl WalletInteractionTransport for Service {
    async fn send_wallet_interaction_response(
        &self,
        session: Session,
        response: WalletToDappInteractionResponse,
    ) -> Result<()> {
        let serialized_response =
            wallet_to_dapp_interaction_response_to_json_bytes(&response);

        let mut encryption_key = session.encryption_key;
        let encrypted_response: Vec<u8> = self
            .encryption_scheme
            .encrypt(serialized_response.to_vec(), &mut encryption_key);

        let hex = hex_encode(encrypted_response);

        let success_response =
            SuccessResponse::new(session.id, session.wallet_public_key, hex);
        let request =
            NetworkRequest::radix_connect_success_response(success_response)?;
        self.http_client.execute_network_request(request).await?;
        Ok(())
    }
}

#[cfg(test)]
impl Service {
    fn new_always_failing() -> Self {
        Self::new_with_network_antenna(Arc::new(
            MockAntenna::new_always_failing(),
        ))
    }

    fn new_succeeding_http_client(request: Vec<u8>) -> Self {
        Self::new_with_network_antenna(Arc::new(MockAntenna::new(200, request)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt::time::timeout;
    use hex::ToHex;
    use std::time::Duration;
    const MAX: Duration = Duration::from_millis(10);

    const SERVICE_PATH: &str =
        "https://radix-connect-relay.radixdlt.com/api/v1";

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
        let mock_antenna = MockAntenna::with_spy(200, (), |request| {
            // Prepare encryption keys
            let mut encryption_key = Session::sample().encryption_key;
            let mut decryption_key = encryption_key;

            // Serialize the response
            let wallet_to_dapp_interaction_response =
                WalletToDappInteractionResponse::sample();
            let body = wallet_to_dapp_interaction_response_to_json_bytes(
                &wallet_to_dapp_interaction_response,
            );

            // Encrypt the response
            let encrypted = EncryptionScheme::default()
                .encrypt(body.to_vec(), &mut encryption_key);
            let hex = hex_encode(encrypted);
            let success_response = SuccessResponse::new(
                SessionID::sample(),
                KeyAgreementPublicKey::sample(),
                hex,
            );

            let encoded = serde_json::to_vec(&success_response).unwrap();

            // Request that is expected to be sent
            let expected_request = NetworkRequest {
                url: Url::from_str(SERVICE_PATH).unwrap(),
                method: NetworkMethod::Post,
                body: encoded.into(),
                headers: HashMap::new(),
            };

            pretty_assertions::assert_eq!(request.url, expected_request.url);
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
                .decrypt(
                    hex_decode(sent_request.data.as_str()).unwrap(),
                    &mut decryption_key,
                )
                .unwrap();
            let decoded_payload: WalletToDappInteractionResponse =
                serde_json::from_slice(&decrypted_payload).unwrap();

            pretty_assertions::assert_eq!(
                decoded_payload,
                wallet_to_dapp_interaction_response
            )
        });

        let service = Service::new_with_network_antenna(Arc::new(mock_antenna));
        let session = Session::sample();

        let req = service.send_wallet_interaction_response(
            session,
            WalletToDappInteractionResponse::sample(),
        );
        let _ = timeout(MAX, req).await.unwrap();
    }
}
