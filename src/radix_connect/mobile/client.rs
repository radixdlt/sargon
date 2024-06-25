use std::borrow::BorrowMut;

use super::deep_link_parsing::*;
use super::relay_service::Service as RelayService;
use super::relay_service::WalletInteractionTransport;
use crate::prelude::*;
use hex::ToHex;
use std::sync::RwLock;

/// The Radix Connect Mobile client that handles the interaction with dApps on mobile through deepLinks.
#[derive(uniffi::Object)]
pub struct RadixConnectMobile {
    /// The transport to be used to send back the response for the WalletInteraction
    wallet_interactions_transport: Arc<dyn WalletInteractionTransport>,
    /// The storage to be used to store the sessions established with dApps.
    session_storage: Arc<dyn SessionStorage>,
    /// The new sessions that have been created and are waiting to be by the users.
    /// Once the session is validated, it will be moved to the secure storage.
    /// Validation consists in verifying the origin of the session.
    new_sessions: RwLock<HashMap<SessionID, Session>>,
}

impl RadixConnectMobile {
    pub fn init(
        wallet_interactions_transport: Arc<dyn WalletInteractionTransport>,
        session_storage: Arc<dyn SessionStorage>,
    ) -> Self {
        Self {
            wallet_interactions_transport,
            session_storage,
            new_sessions: RwLock::new(HashMap::new()),
        }
    }
}

#[uniffi::export]
impl RadixConnectMobile {
    #[uniffi::constructor]
    pub fn new(
        network_antenna: Arc<dyn NetworkAntenna>,
        session_storage: Arc<dyn SessionStorage>,
    ) -> Self {
        Self::init(
            Arc::new(RelayService::new_with_network_antenna(
                network_antenna.clone(),
            )),
            session_storage,
        )
    }
}

#[uniffi::export]
impl RadixConnectMobile {
    #[uniffi::method]
    pub async fn handle_deep_link(
        &self,
        url: String,
    ) -> Result<RadixConnectMobileSessionRequest> {
        // Try to parse the deep link ur to RadixConnectMobileRequest
        let request = parse_mobile_connect_request(url)?;

        // TODO: Move in parse_mobile_connect_request
        request.verify_request_signature()?;

        // A session might be already established for the given session_id.
        let existing_session = self.load_session(request.session_id).await.ok();
        // Each new session requires origin validation.
        let origin_requires_validation = existing_session.is_none();

        match existing_session {
            Some(session) => {
                // Validate the request against the pre-validated session.
                // This is the protection the Wallet can guarantee after the origin was validated by the user.
                session.validate_request(&request)?;
            }
            None => {
                // Don't yet store the session, as it requires validation from the user.
                self.create_in_flight_session(&request)?;
            }
        }

        // Return request to the Wallet to be handled by the user.
        Ok(RadixConnectMobileSessionRequest::new(
            request.session_id,
            request.interaction,
            request.origin,
            origin_requires_validation,
        ))
    }

    #[uniffi::method]
    pub async fn send_dapp_interaction_response(
        &self,
        wallet_response: RadixConnectMobileWalletResponse,
    ) -> Result<()> {
        let session_id = wallet_response.session_id;
        // Get the in flight session, if any, that required validation from the user.
        let in_flight_session = self
            .new_sessions
            .try_write()
            .ok()
            .and_then(|mut new_sessions| new_sessions.remove(&session_id));

        // Get the existing session, if any.
        let existing_session =
            self.load_session(wallet_response.session_id).await.ok();

        let is_in_flight_session = in_flight_session.is_some();
        let session = in_flight_session.or(existing_session).ok_or(
            CommonError::RadixConnectMobileSessionNotFound { session_id },
        )?;

        let is_success_response = wallet_response.response.is_success();

        // Send the wallet interaction response to the dApp through the transport.
        self.wallet_interactions_transport
            .send_wallet_interaction_response(
                session.clone(),
                wallet_response.response,
            )
            .await?;

        if is_in_flight_session && is_success_response {
            // We do consider a session to be validated once user did send a successful interaction back.
            self.save_session(session).await?;
        }

        Ok(())
    }
}

impl RadixConnectMobile {
    const HKDF_KEY_DERIVATION_INFO: &'static str = "RCfM";

    fn create_in_flight_session(
        &self,
        request: &RadixConnectMobileRequest,
    ) -> Result<()> {
        // 1. Generate the Wallet's private/public key pair
        let wallet_private_key = KeyAgreementPrivateKey::generate()?;
        let wallet_public_key = wallet_private_key.public_key();

        // 2. Generate the secret that is shared by the Wallet and the dApp
        let shared_secret = wallet_private_key
            .shared_secret_from_key_agreement(&request.public_key);

        // 3. Derive the encryption key from the shared secret
        let dapp_definition_address = request.dapp_definition_address.address();
        let encryption_key = PbHkdfSha256::hkdf_key_agreement(
            shared_secret.as_bytes(),
            Some(dapp_definition_address.as_bytes()),
            Some(Self::HKDF_KEY_DERIVATION_INFO.as_bytes()),
        );

        // 4. Create the session
        let session = Session::new(
            request.session_id,
            SessionOrigin::WebDapp(request.origin.clone()),
            encryption_key,
            request.public_key,
            request.identity_public_key,
            wallet_public_key,
        );

        // 5. Save the session in memory until validated
        _ = self.new_sessions.try_write().map(|mut new_sessions| {
            new_sessions.insert(request.session_id, session);
        });

        Ok(())
    }
}

impl RadixConnectMobile {
    async fn load_session(&self, session_id: SessionID) -> Result<Session> {
        let session_bytes =
            self.session_storage.load_session(session_id).await?.ok_or(
                CommonError::RadixConnectMobileSessionNotFound { session_id },
            )?;
        deserialize_from_slice(session_bytes.as_slice())
    }

    async fn save_session(&self, session: Session) -> Result<()> {
        let bytes = serialize(&session)?;
        self.session_storage
            .save_session(session.id, bytes.into())
            .await
    }
}
