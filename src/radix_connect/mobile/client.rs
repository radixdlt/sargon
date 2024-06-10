use std::borrow::BorrowMut;

use super::deep_link_parsing::*;
use super::relay_service::Service as RelayService;
use crate::prelude::*;
use std::sync::RwLock;

trait Transport {}

trait SessionStorage {}

trait WalletKeyGenerator {}

/// The Radix Connect Mobile client.
/// This is the object that will be used by the mobile app to handle interactions sent over Radix Connect Relay.
#[derive(uniffi::Object)]
pub struct RadixConnectMobile {
    /// The Radix Connect Relay service to be used to communicate with dApps.
    relay_service: RelayService,
    /// The secure storage to be used to store session data.
    secure_storage: WalletClientStorage,
    /// The client to be used to fetch well-known files.
    well_known_client: WellKnownClient,
    /// The new sessions that have been created and are waiting to be validated on dApp side.
    /// Once the session is validated, it will be moved to the secure storage.
    /// Validation consists in verifying the origin of the session.
    new_sessions: RwLock<HashMap<SessionID, Session>>,
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
            relay_service: RelayService::new_with_network_antenna(
                network_antenna.clone(),
            ),
            secure_storage: WalletClientStorage::new(secure_storage),
            well_known_client: WellKnownClient::new_with_network_antenna(
                network_antenna,
            ),
            new_sessions: RwLock::new(HashMap::new()),
        }
    }

    #[uniffi::method]
    pub async fn handle_linking_request(
        &self,
        request: RadixConnectMobileLinkRequest,
        dev_mode: bool,
    ) -> Result<Url> {
        let wallet_private_key = KeyAgreementPrivateKey::generate()?;
        let shared_secret = wallet_private_key
            .shared_secret_from_key_agreement(&request.public_key);

        let dapp_definitions = self
            .well_known_client
            .get_well_known_file(request.origin.clone())
            .await?;
        // pass dev mode to decide if we want to use default callback path or fail linking
        if !dev_mode && dapp_definitions.callback_path.is_none() {
            return Err(
                CommonError::RadixConnectMobileDappCallbackPathNotFound {
                    origin: request.origin,
                },
            );
        }
        let callback_path = dapp_definitions.callback_path.unwrap_or_default();

        let salt = hex_decode("000102030405060708090a0b0c0d0e0f").unwrap();
        let info = hex_decode("f0f1f2f3f4f5f6f7f8f9").unwrap();

        let encryption_key = PbHkdfSha256::hkdf_key_agreement(
            shared_secret.to_bytes(),
            Some(&salt),
            Some(&info),
        );

        let session = Session::new(
            request.session_id,
            SessionOrigin::WebDapp(request.origin.clone()),
            encryption_key,
            callback_path.clone(),
        );

        {
            self.new_sessions
                .try_write()
                .map(|mut new_sessions| {
                    new_sessions.insert(request.session_id, session);
                })
                .unwrap();
        }

        let mut return_url = request.origin.join(&callback_path.0).unwrap();
        return_url
            .query_pairs_mut()
            .append_pair("sessionID", request.session_id.0.as_str())
            .append_pair(
                "publicKey",
                wallet_private_key.public_key().to_hex().as_str(),
            );

        Ok(return_url)
    }

    #[uniffi::method]
    pub async fn handle_dapp_interaction_request(
        &self,
        dapp_request: RadixConnectMobileDappRequest,
    ) -> Result<RadixConnectMobileSessionRequest> {
        let session;
        {
            let new_sessions = self.new_sessions.try_read().unwrap();

            session = match new_sessions.get(&dapp_request.session_id) {
                Some(session) => {
                    // Save the session to the secure storage
                    self.secure_storage.save_session(session.clone())?;
                    let session_id = session.id;

                    self.new_sessions
                        .try_write()
                        .map(|mut new_sessions| {
                            new_sessions.remove(&session_id);
                        })
                        .unwrap();

                    session.to_owned()
                }
                None => {
                    self.session_from_secure_storage(dapp_request.session_id)?
                }
            };
        }

        self.relay_service
            .get_wallet_interaction_requests(session)
            .await?
            .into_iter()
            .find(|intraction| {
                intraction.interaction_id == dapp_request.interaction_id
            })
            .map(|interaction| RadixConnectMobileSessionRequest {
                session_id: dapp_request.session_id,
                interaction,
            })
            .ok_or(CommonError::RadixConnectMobileDappRequestNotFound {
                interaction_id: dapp_request.interaction_id,
            })
    }

    #[uniffi::method]
    pub async fn send_dapp_interaction_response(
        &self,
        wallet_response: RadixConnectMobileWalletResponse,
    ) -> Result<()> {
        let session =
            self.session_from_secure_storage(wallet_response.session_id)?;

        self.relay_service
            .send_wallet_interaction_response(session, wallet_response.response)
            .await
    }
}

impl RadixConnectMobile {
    fn session_from_secure_storage(
        &self,
        session_id: SessionID,
    ) -> Result<Session> {
        self.secure_storage.load_session(session_id)?.ok_or(
            CommonError::RadixConnectMobileSessionNotFound { session_id },
        )
    }
}

impl WalletClientStorage {
    fn save_session(&self, session: Session) -> Result<()> {
        self.save(
            SecureStorageKey::RadixConnectMobileSession {
                session_id: session.id,
            },
            &session,
        )
    }

    fn load_session(&self, session_id: SessionID) -> Result<Option<Session>> {
        self.load(SecureStorageKey::RadixConnectMobileSession { session_id })
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
