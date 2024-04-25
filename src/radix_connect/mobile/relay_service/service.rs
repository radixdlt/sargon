use super::super::session::Session;
use super::super::session_id::SessionID;
use super::super::session_origin::SessionOrigin;
use super::models::*;
use crate::prelude::*;

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

    async fn get_requests(
        &self,
        session: Session,
    ) -> Result<Vec<DappToWalletInteractionUnvalidated>> {
        let request = NetworkRequest::radix_connect_relay_request(
            Request::get_requests(session.id),
        )?;
        let response: Vec<Vec<u8>> = self
            .http_client
            .execute_request_with_decoding(request)
            .await?;

        let mut encryption_key = session.encryption_key;
        let decrypted = response
            .iter()
            .map(|bytes| {
                self.encryption_scheme
                    .decrypt(bytes.to_vec(), &mut encryption_key)
            })
            .collect::<Result<Vec<_>>>()?;

        decrypted
            .iter()
            .map(|bytes| {
                new_dapp_to_wallet_interaction_unvalidated_from_json_bytes(
                    bytes.clone().into(),
                )
            })
            .collect()
    }

    async fn send_response(
        &self,
        session: Session,
        response: WalletToDappInteractionResponse,
    ) -> Result<()> {
        let body = wallet_to_dapp_interaction_response_to_json_bytes(&response);
        let mut encryption_key = session.encryption_key;
        let encrypted = self
            .encryption_scheme
            .encrypt(body.to_vec(), &mut encryption_key);
        let request = NetworkRequest::radix_connect_relay_request(
            Request::send_response(session.id, encrypted),
        )?;
        self.http_client.execute_network_request(request).await?;
        Ok(())
    }

    async fn get_handshake_request(
        &self,
        session_id: SessionID,
    ) -> Result<HandshakeRequest> {
        let request = NetworkRequest::radix_connect_relay_request(
            Request::get_handshake_request(session_id),
        )?;
        self.http_client
            .execute_request_with_decoding(request)
            .await
    }

    async fn send_handshake_response(
        &self,
        session_id: SessionID,
        public_key: impl Into<PublicKey>,
    ) -> Result<()> {
        let body = BagOfBytes::from_hex(public_key.into().to_hex().as_str())?;
        let request = NetworkRequest::radix_connect_relay_request(
            Request::send_handshake_response(session_id, body),
        )?;
        self.http_client.execute_network_request(request).await?;
        Ok(())
    }
}

impl NetworkRequest {
    fn radix_connect_relay_request(request: Request) -> Result<Self> {
        NetworkRequest::new_post(
            Url::from_str("https://radix-connect-relay-dev.rdx-works-main.extratools.works/api/v1").unwrap()
        )
        .with_serializing_body(request)
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
    async fn test_get_requests() {
        let mut encryption_key = Exactly32Bytes::generate();
        let decryption_key = encryption_key;

        let dapp_to_wallet_interaction_unvalidated =
            DappToWalletInteractionUnvalidated::sample();
        let body = dapp_to_wallet_interaction_unvalidated_to_json_bytes(
            &dapp_to_wallet_interaction_unvalidated,
        );
        let encrypted = EncryptionScheme::default()
            .encrypt(body.to_vec(), &mut encryption_key);
        let requests = vec![encrypted];
        let encoded_requests = serde_json::to_vec(&requests).unwrap();

        let mock_antenna = MockAntenna::new(200, encoded_requests);
        let http_client = HttpClient {
            network_antenna: Arc::new(mock_antenna),
        };
        let service = Service::new(http_client);

        let session = Session {
            id: SessionID::sample(),
            origin: SessionOrigin::WebDapp(
                Url::from_str("https://example.com").unwrap(),
            ),
            encryption_key: decryption_key,
        };

        let req = service.get_requests(session);
        let result = timeout(MAX, req).await.unwrap();
        pretty_assertions::assert_eq!(
            result,
            Ok(vec![dapp_to_wallet_interaction_unvalidated])
        );
    }

    #[actix_rt::test]
    async fn test_send_response() {
        let mock_antenna = MockAntenna::with_spy(200, (), |request| {
            let mut encryption_key = Exactly32Bytes::sample();
            let mut decryption_key = encryption_key;

            let wallet_to_dapp_interaction_response =
                WalletToDappInteractionResponse::sample();
            let body = wallet_to_dapp_interaction_response_to_json_bytes(
                &wallet_to_dapp_interaction_response,
            );

            let encrypted = EncryptionScheme::default()
                .encrypt(body.to_vec(), &mut encryption_key);
            let relay_request =
                Request::send_response(SessionID::sample(), encrypted.clone());
            let encoded = serde_json::to_vec(&relay_request).unwrap();

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

        let http_client = HttpClient {
            network_antenna: Arc::new(mock_antenna),
        };

        let service = Service::new(http_client);

        let session = Session {
            id: SessionID::sample(),
            origin: SessionOrigin::WebDapp(
                Url::from_str("https://example.com").unwrap(),
            ),
            encryption_key: Exactly32Bytes::sample(),
        };

        let req = service
            .send_response(session, WalletToDappInteractionResponse::sample());
        let _ = timeout(MAX, req).await.unwrap();
    }

    #[actix_rt::test]
    async fn test_get_handshake_request() {
        let request = HandshakeRequest::sample();
        let mock_antenna =
            MockAntenna::new(200, serde_json::to_vec(&request).unwrap());

        let http_client = HttpClient {
            network_antenna: Arc::new(mock_antenna),
        };

        let service = Service::new(http_client);

        let session_id = SessionID::sample();

        let req = service.get_handshake_request(session_id);
        let result = timeout(MAX, req).await.unwrap();
        pretty_assertions::assert_eq!(result, Ok(request));
    }

    #[actix_rt::test]
    async fn test_send_handshake_response() {
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

        let http_client = HttpClient {
            network_antenna: Arc::new(mock_antenna),
        };

        let service = Service::new(http_client);
        let session_id = SessionID::sample();

        let req =
            service.send_handshake_response(session_id, PublicKey::sample());
        let _ = timeout(MAX, req).await.unwrap();
    }
}
