use super::deep_link_parsing::*;
use super::relay_service::Service as RelayService;
use crate::prelude::*;

/// The Radix Connect Mobile client.
/// This is the object that will be used by the mobile app to handle interactions sent over Radix Connect Relay.
#[derive(uniffi::Object)]
pub struct RadixConnectMobile {
    relay_service: RelayService,
    secure_storage: Arc<dyn SecureStorage>,
}

// Provisional API
#[uniffi::export]
impl RadixConnectMobile {
    // RadixConnectMobile should require a NetworkAntenna and a SecureStorage from the Wallet.
    // The internal components, such as RadixConnectRelayService will be created by the RadixConnectMobile.
    #[uniffi::constructor]
    pub fn new(
        network_antenna: Arc<dyn NetworkAntenna>,
        secure_storage: Arc<dyn SecureStorage>,
    ) -> Self {
        Self {
            relay_service: RelayService::new(HttpClient { network_antenna }),
            secure_storage,
        }
    }

    // #[uniffi::method]
    // pub async fn handle_linking_request(
    //     &self,
    //     request: RadixConnectMobileLinkRequest,
    // ) -> Result<Url> {
    //     // Steps
    //     // 1. Fetch the handshake request from relay service for the session id
    //     // 2. Generate a Curve25519 key for Diffie-Hellman agreement
    //     // 3. Generate a Curve25519 key agreement public key
    //     // 4. Generate a Curve25519 shared secret by using the private key and the public key from the handshake request
    //     // 5. Derive the encryption key with HKDF using the shared secret and the constant salt
    //     // 6. Upload the public key to the relay service using the handshake response
    //     // 7. Create the Session
    //     // 8. Store the session in the secure storage
    //     // 9. Return the URL to the mobile app

    //     // 1. Fetch the handshake request from relay service for the session id
    //     let handshake_request = self.relay_service.get_session_handshake_request(request.session_id).await?;

    //     // 2. Generate a Curve25519 key for Diffie-Hellman agreement
    //     let wallet_secret = EphemeralSecret::random();
    //     let public_key = PublicKey::from(&wallet_secret);

    //     let shared_secret = wallet_secret.diffie_hellman(&handshake_request.public_key);

    //     Ok(Url::parse("https://example.com").unwrap())
    // }

    #[uniffi::method]
    pub fn handle_dapp_interaction_request(
        &self,
        _request: RadixConnectMobileDappRequest,
    ) -> Result<DappToWalletInteraction> {
        todo!()
    }

    #[uniffi::method]
    pub fn send_dapp_interaction_response(
        &self,
        _response: WalletToDappInteractionResponse,
    ) -> Result<Url> {
        todo!()
    }
}

#[uniffi::export]
pub fn new_mobile_connect_request(
    url: String,
) -> Result<RadixConnectMobileConnectRequest> {
    RadixConnectMobileConnectRequest::from_str(url.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_mobile_connect_request() {
        let uuid = Uuid::new_v4().to_string();
        let connect_url = format!("https://d1rxdfxrfmemlj.cloudfront.net/?sessionId={}&origin=radix%3A%2F%2Fapp", uuid);
        assert!(new_mobile_connect_request(connect_url).is_ok());
    }
}
