use super::super::session::*;
use super::models::*;
use crate::prelude::*;

/// The service that interacts with the Radix Connect Relay.
/// API docs at https://github.com/radixdlt/radix-connect-relay?tab=readme-ov-file#api-v1
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

const SERVICE_PATH: &str =
    "https://radix-connect-relay-dev.rdx-works-main.extratools.works/api/v1";

impl NetworkRequest {
    fn radix_connect_relay_request(request: Request) -> Result<Self> {
        NetworkRequest::new_post(Url::from_str(SERVICE_PATH).unwrap())
            .with_serializing_body(request)
    }
}

impl Service {
    async fn get_wallet_interaction_requests(
        &self,
        session: Session,
    ) -> Result<Vec<DappToWalletInteractionUnvalidated>> {
        let request = NetworkRequest::radix_connect_relay_request(
            Request::new_get_requests(session.id),
        )?;

        let encrypted_wallet_interactions: Vec<Vec<u8>> = self
            .http_client
            .execute_request_with_decoding(request)
            .await?;

        let mut encryption_key = session.encryption_key;
        let decrypted_wallet_interactions = encrypted_wallet_interactions
            .iter()
            .map(|bytes| {
                self.encryption_scheme
                    .decrypt(bytes.to_vec(), &mut encryption_key)
            })
            .collect::<Result<Vec<_>>>()?;

        let deserialized_wallet_interactions_result =
            decrypted_wallet_interactions
                .iter()
                .map(|bytes| {
                    new_dapp_to_wallet_interaction_unvalidated_from_json_bytes(
                        bytes.clone().into(),
                    )
                })
                .collect();

        deserialized_wallet_interactions_result
    }

    async fn send_wallet_interaction_response(
        &self,
        session: Session,
        response: WalletToDappInteractionResponse,
    ) -> Result<()> {
        let serialized_response =
            wallet_to_dapp_interaction_response_to_json_bytes(&response);

        let mut encryption_key = session.encryption_key;
        let encrypted_response = self
            .encryption_scheme
            .encrypt(serialized_response.to_vec(), &mut encryption_key);

        let request = NetworkRequest::radix_connect_relay_request(
            Request::new_send_response(session.id, encrypted_response),
        )?;
        self.http_client.execute_network_request(request).await?;
        Ok(())
    }

    async fn get_session_handshake_request(
        &self,
        session_id: SessionID,
    ) -> Result<SessionHandshakeRequest> {
        let request = NetworkRequest::radix_connect_relay_request(
            Request::new_get_handshake_request(session_id),
        )?;
        self.http_client
            .execute_request_with_decoding(request)
            .await
    }

    async fn send_session_handshake_response(
        &self,
        session_id: SessionID,
        public_key: impl Into<PublicKey>,
    ) -> Result<()> {
        let request = NetworkRequest::radix_connect_relay_request(
            Request::new_send_handshake_response(
                session_id,
                public_key.into().to_bytes(),
            ),
        )?;
        self.http_client.execute_network_request(request).await?;
        Ok(())
    }
}

#[cfg(test)]
impl Service {
    fn new_always_failing() -> Self {
        Self::new_with_network_antenna(Arc::new(
            MockAntenna::new_always_failling(),
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
    use std::time::Duration;
    const MAX: Duration = Duration::from_millis(10);

    const SERVICE_PATH: &str = "https://radix-connect-relay-dev.rdx-works-main.extratools.works/api/v1";

    #[actix_rt::test]
    async fn test_get_wallet_interaction_requests_correct_request_is_made() {
        let mock_antenna_with_spy =
            MockAntenna::with_spy(200, vec![], |request| {
                let relay_request =
                    Request::new_get_requests(Session::sample().id);
                let encoded_request =
                    serde_json::to_vec(&relay_request).unwrap();

                let expected_request = NetworkRequest {
                    url: Url::from_str(SERVICE_PATH).unwrap(),
                    method: NetworkMethod::Post,
                    body: encoded_request.into(),
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
                pretty_assertions::assert_eq!(
                    request.body,
                    expected_request.body
                );
            });

        let service =
            Service::new_with_network_antenna(Arc::new(mock_antenna_with_spy));
        let session = Session::sample();

        let req = service.get_wallet_interaction_requests(session);
        let _ = timeout(MAX, req).await.unwrap();
    }

    #[actix_rt::test]
    async fn test_get_wallet_interaction_requests_failure() {
        let service = Service::new_always_failing();
        let session = Session::sample();

        let req = service.get_wallet_interaction_requests(session);
        let result = timeout(MAX, req).await.unwrap();
        assert!(result.is_err());
    }

    #[actix_rt::test]
    async fn test_get_wallet_interaction_requests() {
        let session = Session::sample();
        // Prepare encryption keys
        let mut encryption_key = session.encryption_key.clone();

        // Serialize the request
        let dapp_to_wallet_interaction_unvalidated =
            DappToWalletInteractionUnvalidated::sample();
        let body = dapp_to_wallet_interaction_unvalidated_to_json_bytes(
            &dapp_to_wallet_interaction_unvalidated,
        );

        // Encrypt the request
        let encrypted = EncryptionScheme::default()
            .encrypt(body.to_vec(), &mut encryption_key);
        let requests = vec![encrypted];
        let encoded_requests = serde_json::to_vec(&requests).unwrap();

        // Setup Service
        let service = Service::new_succeeding_http_client(encoded_requests);

        // Get interaction requests
        let req = service.get_wallet_interaction_requests(session);
        let result = timeout(MAX, req).await.unwrap();

        // Assert the result
        pretty_assertions::assert_eq!(
            result,
            Ok(vec![dapp_to_wallet_interaction_unvalidated])
        );
    }

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
            let mut encryption_key = Session::sample().encryption_key.clone();
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
            let relay_request = Request::new_send_response(
                SessionID::sample(),
                encrypted.clone(),
            );
            let encoded = serde_json::to_vec(&relay_request).unwrap();

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

            let sent_request: Request =
                serde_json::from_slice(&expected_request.body).unwrap();
            pretty_assertions::assert_eq!(
                sent_request.session_id,
                relay_request.session_id
            );
            pretty_assertions::assert_eq!(
                sent_request.method,
                relay_request.method
            );

            let decrypted_payload = EncryptionScheme::default()
                .decrypt(
                    sent_request.data.unwrap().to_vec(),
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

    #[actix_rt::test]
    async fn get_session_handshake_request_correct_request_is_made() {
        let mock_antenna_with_spy =
            MockAntenna::with_spy(200, vec![], |request| {
                let session_id = SessionID::sample();
                let relay_request =
                    Request::new_get_handshake_request(session_id);
                let encoded_request =
                    serde_json::to_vec(&relay_request).unwrap();

                let expected_request = NetworkRequest {
                    url: Url::from_str(SERVICE_PATH).unwrap(),
                    method: NetworkMethod::Post,
                    body: encoded_request.into(),
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
                pretty_assertions::assert_eq!(
                    request.body,
                    expected_request.body
                );
            });

        let service =
            Service::new_with_network_antenna(Arc::new(mock_antenna_with_spy));
        let session_id = SessionID::sample();

        let req = service.get_session_handshake_request(session_id);
        let _ = timeout(MAX, req).await.unwrap();
    }

    #[actix_rt::test]
    async fn get_session_handshake_request_failure() {
        let service = Service::new_always_failing();
        let session_id = SessionID::sample();

        let req = service.get_session_handshake_request(session_id);
        let result = timeout(MAX, req).await.unwrap();
        assert!(result.is_err());
    }

    #[actix_rt::test]
    async fn test_get_session_handshake_request() {
        let request = SessionHandshakeRequest::sample();
        let mock_antenna =
            MockAntenna::new(200, serde_json::to_vec(&request).unwrap());

        let service = Service::new_with_network_antenna(Arc::new(mock_antenna));

        let session_id = SessionID::sample();

        let req = service.get_session_handshake_request(session_id);
        let result = timeout(MAX, req).await.unwrap();
        pretty_assertions::assert_eq!(result, Ok(request));
    }

    #[actix_rt::test]
    async fn test_send_session_handshake_response_failure() {
        let service = Service::new_always_failing();
        let session_id = SessionID::sample();

        let req = service
            .send_session_handshake_response(session_id, PublicKey::sample());
        let result = timeout(MAX, req).await.unwrap();
        assert!(result.is_err());
    }

    #[actix_rt::test]
    async fn test_send_session_handshake_response() {
        let mock_antenna = MockAntenna::with_spy(200, (), |request| {
            let public_key = PublicKey::sample();
            let body = Request::new(
                Method::SendHandshakeResponse,
                SessionID::sample(),
                BagOfBytes::from_hex(public_key.to_hex().as_str()).unwrap(),
            );

            let encoded = serde_json::to_vec(&body).unwrap();

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
            pretty_assertions::assert_eq!(request.body, expected_request.body);
        });

        let service = Service::new_with_network_antenna(Arc::new(mock_antenna));
        let session_id = SessionID::sample();

        let req = service
            .send_session_handshake_response(session_id, PublicKey::sample());
        let _ = timeout(MAX, req).await.unwrap();
    }
}