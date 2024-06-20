use std::borrow::BorrowMut;

use super::deep_link_parsing::*;
use super::relay_service::Service as RelayService;
use super::relay_service::WalletInteractionTransport;
use crate::prelude::*;
use hex::ToHex;
use std::sync::RwLock;

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait SessionStorage: Send + Sync {
    async fn save_session(
        &self,
        session_id: SessionID,
        encoded_session: BagOfBytes,
    ) -> Result<()>;

    async fn load_session(
        &self,
        session_id: SessionID,
    ) -> Result<Option<BagOfBytes>>;
}

trait WalletKeyGenerator {}

/// The Radix Connect Mobile client.
/// This is the object that will be used by the mobile app to handle interactions sent over Radix Connect Relay.
#[derive(uniffi::Object)]
pub struct RadixConnectMobile {
    /// The Radix Connect Relay service to be used to communicate with dApps.
    wallet_interactions_transport: Arc<dyn WalletInteractionTransport>,
    /// The storage to be used to store session data.
    session_storage: Arc<dyn SessionStorage>,
    /// The new sessions that have been created and are waiting to be validated on dApp side.
    /// Once the session is validated, it will be moved to the secure storage.
    /// Validation consists in verifying the origin of the session.
    new_sessions: RwLock<HashMap<SessionID, Session>>,
}

#[uniffi::export]
impl RadixConnectMobile {
    // RadixConnectMobile should require a NetworkAntenna and a SecureStorage from the Wallet.
    // The internal components, such as RadixConnectRelayService will be created by the RadixConnectMobile.
    #[uniffi::constructor]
    pub fn new(
        network_antenna: Arc<dyn NetworkAntenna>,
        session_storage: Arc<dyn SessionStorage>,
    ) -> Self {
        Self {
            wallet_interactions_transport: Arc::new(
                RelayService::new_with_network_antenna(network_antenna.clone()),
            ),
            session_storage,
            new_sessions: RwLock::new(HashMap::new()),
        }
    }
}

#[uniffi::export]
impl RadixConnectMobile {
    #[uniffi::method]
    pub async fn handle_deep_link(
        &self,
        url: String,
    ) -> Result<RadixConnectMobileSessionRequest> {
        let request = parse_mobile_connect_request(url)?;

        request
            .verify_request_signature(&request.interaction.interaction_id)?;

        let existing_session = self.load_session(request.session_id).await.ok();
        let origin_requires_validation = existing_session.is_none();

        match existing_session {
            Some(session) => {
                session.validate_request(&request)?;
            }
            None => {
                self.create_in_flight_session(&request)?;
            }
        }

        Ok(RadixConnectMobileSessionRequest::new(
            request.session_id,
            request.interaction,
            request.origin,
            origin_requires_validation,
        ))
    }

    #[uniffi::method]
    pub async fn request_origin_verified(
        &self,
        session_id: SessionID,
    ) -> Result<()> {
        match self
            .new_sessions
            .try_write()
            .ok()
            .and_then(|mut new_sessions| new_sessions.remove(&session_id))
        {
            Some(inflight_session) => self.save_session(inflight_session).await,
            None => Err(CommonError::Unknown),
        }
    }

    #[uniffi::method]
    pub async fn request_origin_denied(
        &self,
        session_id: SessionID,
    ) -> Result<()> {
        {
            if let Ok(mut new_sessions) = self.new_sessions.try_write() {
                new_sessions.remove(&session_id);
            }
        }

        self.wallet_interactions_transport
            .send_wallet_interaction_error_response(
                session_id,
                "Rejected".to_owned(),
            )
            .await
    }

    //
    //     let encryption_key: Exactly32Bytes = shared_secret.as_bytes().into();
    //     // let salt = hex_decode("000102030405060708090a0b0c0d0e0f").unwrap();
    //     // let info = hex_decode("f0f1f2f3f4f5f6f7f8f9").unwrap();
    //
    //     // let encryption_key = PbHkdfSha256::hkdf_key_agreement(
    //     //     shared_secret.to_bytes(),
    //     //     Some(&salt),
    //     //     Some(&info),
    //     // );
    //

    #[uniffi::method]
    pub async fn send_dapp_interaction_response(
        &self,
        wallet_response: RadixConnectMobileWalletResponse,
    ) -> Result<()> {
        let session_id = wallet_response.session_id;
        let in_flight_session = self
            .new_sessions
            .try_write()
            .ok()
            .and_then(|mut new_sessions| new_sessions.remove(&session_id));

        let existing_session =
            self.load_session(wallet_response.session_id).await.ok();

        let is_in_flight_session = in_flight_session.is_some();
        let session = in_flight_session.or(existing_session).ok_or(
            CommonError::RadixConnectMobileSessionNotFound { session_id },
        )?;

        if is_in_flight_session && !wallet_response.response.is_success() {
            return self
                .wallet_interactions_transport
                .send_wallet_interaction_error_response(
                    session_id,
                    "Failure".to_string(),
                )
                .await;
        }

        self.wallet_interactions_transport
            .send_wallet_interaction_response(
                session.clone(),
                wallet_response.response,
            )
            .await?;

        if is_in_flight_session {
            self.save_session(session).await?;
        }

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

    fn create_in_flight_session(
        &self,
        request: &RadixConnectMobileRequest,
    ) -> Result<()> {
        let wallet_private_key = KeyAgreementPrivateKey::generate()?;
        let wallet_public_key = wallet_private_key.public_key();
        let shared_secret = wallet_private_key
            .shared_secret_from_key_agreement(&request.public_key);

        let encryption_key: Exactly32Bytes = shared_secret.as_bytes().into();
        let session = Session::new(
            request.session_id,
            SessionOrigin::WebDapp(request.origin.clone()),
            encryption_key,
            request.public_key,
            request.identity_public_key,
            wallet_public_key,
        );

        _ = self.new_sessions.try_write().map(|mut new_sessions| {
            new_sessions.insert(request.session_id, session);
        });

        Ok(())
    }
}
