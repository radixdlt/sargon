use drivers::NetworkingDriver;
use encryption::PbHkdfSha256;

use super::deep_link_parsing::*;
use super::relay_service::Service as RelayService;
use super::relay_service::WalletInteractionTransport;
use crate::prelude::*;
use std::sync::RwLock;

/// The Radix Connect Mobile client that handles the interaction with dApps on mobile through deepLinks.
pub struct RadixConnectMobile {
    /// The transport to be used to send back the response for the WalletInteraction
    wallet_interactions_transport: Arc<dyn WalletInteractionTransport>,

    /// The storage to be used to store the sessions established with dApps.
    session_storage: Arc<dyn RadixConnectMobileSessionStorage>,

    /// The new sessions that have been created and are waiting to be by the users.
    /// Once the session is validated, it will be moved to the secure storage.
    /// Validation consists in verifying the origin of the session.
    /// There can be multiple sessions in flight for the same session id, for the scenario
    /// when users send multiple requests to the same session without validating the first one.
    new_sessions:
        RwLock<HashMap<SessionID, HashMap<WalletInteractionId, Session>>>,
}

impl RadixConnectMobile {
    pub fn init(
        wallet_interactions_transport: Arc<dyn WalletInteractionTransport>,
        session_storage: Arc<dyn RadixConnectMobileSessionStorage>,
    ) -> Self {
        Self {
            wallet_interactions_transport,
            session_storage,
            new_sessions: RwLock::new(HashMap::new()),
        }
    }
}

impl RadixConnectMobile {
    pub fn new(
        networking_driver: Arc<dyn NetworkingDriver>,
        session_storage: Arc<dyn RadixConnectMobileSessionStorage>,
    ) -> Self {
        Self::init(
            Arc::new(RelayService::new_with_networking_driver(
                networking_driver.clone(),
            )),
            session_storage,
        )
    }
}

impl RadixConnectMobile {
    /// Try to parse the deep link and create a RadixConnectMobileDappRequest.
    /// This is a stateful operation as it will create an in flight session, that needs to be validated by the user.
    pub async fn handle_deep_link(
        &self,
        url: String,
    ) -> Result<RadixConnectMobileSessionRequest> {
        // Try to parse the deep link ur to RadixConnectMobileDappRequest
        let request = parse_mobile_connect_request(url)?;

        // A session might be already established for the given session_id.
        let existing_session = self.load_session(request.session_id).await.ok();
        // Each new session requires origin validation.
        let origin_requires_validation = existing_session.is_none();

        match existing_session {
            Some(session) => {
                // Validate the request against the pre-validated session.
                // This is the protection the Host can guarantee after the origin was validated by the user.
                session.validate_request(&request)?;
            }
            None => {
                // Don't yet store the session, as it requires validation from the user.
                self.create_in_flight_session(&request)?;
            }
        }

        // Return request to the Host to be handled by the user.
        Ok(RadixConnectMobileSessionRequest::new(
            request.session_id,
            request.interaction,
            request.origin,
            origin_requires_validation,
        ))
    }

    /// Send the Host's response to the dApp.
    /// This is a stateful operation as it will save the session in the secure storage if the user validated the session.
    pub async fn send_dapp_interaction_response(
        &self,
        wallet_response: RadixConnectMobileWalletResponse,
    ) -> Result<()> {
        let session_id = wallet_response.session_id;
        let interaction_id = wallet_response.response.interaction_id();

        // Get the in flight session, if any, that required validation from the user.
        // Not removed at this stage, as the user might retry the interaction.
        let in_flight_session =
            self.new_sessions.read().ok().and_then(|sessions| {
                let sessions_for_id = sessions.get(&session_id).cloned();

                sessions_for_id.and_then(|session| {
                    session
                        .get(&wallet_response.response.interaction_id())
                        .cloned()
                })
            });

        // Get the existing session, if any.
        let existing_session =
            self.load_session(wallet_response.session_id).await.ok();

        let is_in_flight_session = in_flight_session.is_some();
        let session = existing_session.or(in_flight_session).ok_or(
            CommonError::RadixConnectMobileSessionNotFound {
                session_id: session_id.to_string(),
            },
        )?;

        let is_success_response = wallet_response.response.is_success();

        // Send the wallet interaction response to the dApp through the transport.
        self.wallet_interactions_transport
            .send_wallet_interaction_response(
                session.clone(),
                wallet_response.response,
            )
            .await?;

        // Remove the in flight session from the in flight sessions only after it was successfully saved.
        _ = self.new_sessions.try_write().map(|mut new_sessions| {
            let sessions_for_interaction_id = new_sessions
                .get_mut(&session_id)
                .map(|sessions_for_interaction_id| {
                    sessions_for_interaction_id.remove(&interaction_id);
                    sessions_for_interaction_id.clone()
                });

            if sessions_for_interaction_id
                .is_some_and(|sessions| sessions.is_empty())
            {
                new_sessions.remove(&session_id);
            }
        });

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
        request: &RadixConnectMobileDappRequest,
    ) -> Result<()> {
        // 1. Generate the Host's private/public key pair
        let wallet_private_key = KeyAgreementPrivateKey::generate()?;
        let wallet_public_key = wallet_private_key.public_key();

        // 2. Generate the secret that is shared by the Host and the dApp
        let shared_secret = wallet_private_key
            .shared_secret_from_key_agreement(&request.public_key);

        // 3. Derive the encryption key from the shared secret
        let dapp_definition_address = request.dapp_definition_address.address();
        let encryption_key = PbHkdfSha256::derive_key(
            shared_secret.as_bytes(),
            dapp_definition_address.as_bytes(),
            Self::HKDF_KEY_DERIVATION_INFO.as_bytes(),
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
        self.new_sessions
            .try_write()
            .map(|mut new_sessions| {
                new_sessions.entry(request.session_id).or_default().insert(
                    request.interaction.interaction_id.clone(),
                    session,
                );
            })
            .map_err(|_| {
                CommonError::RadixConnectMobileFailedToCreateNewSession
            })
    }
}

impl RadixConnectMobile {
    async fn load_session(&self, session_id: SessionID) -> Result<Session> {
        let session_bytes =
            self.session_storage.load_session(session_id).await?.ok_or(
                CommonError::RadixConnectMobileSessionNotFound {
                    session_id: session_id.to_string(),
                },
            )?;
        session_bytes.deserialize()
    }

    async fn save_session(&self, session: Session) -> Result<()> {
        let bytes = session.serialize_to_bytes()?;
        self.session_storage
            .save_session(session.id, bytes.into())
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    struct MockWalletInteractionTransport {
        stubed_result: Result<()>,
        responses: Arc<Mutex<Vec<(Session, WalletToDappInteractionResponse)>>>,
    }

    impl MockWalletInteractionTransport {
        fn new(stubed_result: Result<()>) -> Self {
            Self {
                stubed_result,
                responses: Arc::new(Mutex::new(Vec::new())),
            }
        }

        fn succeeding() -> Self {
            Self::new(Ok(()))
        }
    }

    #[async_trait::async_trait]
    impl WalletInteractionTransport for MockWalletInteractionTransport {
        async fn send_wallet_interaction_response(
            &self,
            session: Session,
            response: WalletToDappInteractionResponse,
        ) -> Result<()> {
            self.responses.lock().unwrap().push((session, response));
            self.stubed_result.clone()
        }
    }

    struct MockSessionStorage {
        sessions: Arc<Mutex<HashMap<SessionID, Session>>>,
    }

    impl MockSessionStorage {
        fn new() -> Self {
            Self {
                sessions: Arc::new(Mutex::new(HashMap::new())),
            }
        }
    }

    #[async_trait::async_trait]
    impl RadixConnectMobileSessionStorage for MockSessionStorage {
        async fn save_session(
            &self,
            session_id: SessionID,
            encoded_session: BagOfBytes,
        ) -> Result<()> {
            self.sessions
                .lock()
                .unwrap()
                .insert(session_id, encoded_session.deserialize()?);
            Ok(())
        }

        async fn load_session(
            &self,
            session_id: SessionID,
        ) -> Result<Option<BagOfBytes>> {
            self.sessions
                .lock()
                .unwrap()
                .get(&session_id)
                .map(|session| {
                    let encoded_session = session.serialize_to_bytes().unwrap();
                    Some(encoded_session.into())
                })
                .ok_or(CommonError::RadixConnectMobileSessionNotFound {
                    session_id: session_id.to_string(),
                })
        }
    }

    #[allow(clippy::upper_case_acronyms)]
    type SUT = RadixConnectMobile;

    #[actix_rt::test]
    async fn test_invalid_deep_link() {
        let sut = SUT::init(
            Arc::new(MockWalletInteractionTransport::succeeding()),
            Arc::new(MockSessionStorage::new()),
        );

        let url = "https://bad.com";
        let result = sut.handle_deep_link(url.to_string()).await;

        pretty_assertions::assert_eq!(result.is_err(), true)
    }

    // Test that the proper session is created when the dApp sends a request.
    // It would contain the correct dApp informattion and the encryption key derived from the shared secret.
    #[actix_rt::test]
    async fn test_happy_path_first_session_is_properly_created() {
        let mock_session_storage = Arc::new(MockSessionStorage::new());
        let sut = SUT::init(
            Arc::new(MockWalletInteractionTransport::succeeding()),
            mock_session_storage.clone(),
        );

        // Emulate the dApp side by creating the dApp specific key pair
        let dapp_private_key = KeyAgreementPrivateKey::generate().unwrap();
        let dapp_public_key = dapp_private_key.public_key();

        // Create the request params with the dApp public key
        let mut request_params = SampleRequestParams::new_from_text_vector();
        request_params.public_key = dapp_public_key.to_hex().into();

        // Handle the deep link
        _ = sut
            .handle_deep_link(request_params.build_base_url().to_string())
            .await
            .unwrap();

        let session_id = SessionID(
            request_params.session_id.clone().unwrap().parse().unwrap(),
        );

        let interaction_id =
            SampleRequestParams::test_vector_encoded_interaction()
                .interaction_id
                .clone();

        // Assert that a new session was created and stored in memory
        let created_session = sut
            .new_sessions
            .read()
            .unwrap()
            .get(&session_id)
            .unwrap()
            .get(&interaction_id)
            .cloned();

        pretty_assertions::assert_eq!(created_session.is_some(), true);
        let created_session = created_session.unwrap();

        // Generate the shared secret on the dApp side
        let wallet_public_key = created_session.wallet_public_key;
        let shared_secret = dapp_private_key
            .shared_secret_from_key_agreement(&wallet_public_key);

        // Derive the encryption key from the shared secret
        let expected_encryption_key = PbHkdfSha256::derive_key(
            shared_secret.as_bytes(),
            request_params
                .dapp_definition_address
                .clone()
                .unwrap()
                .as_bytes(),
            "RCfM".as_bytes(),
        );

        // Create the expected session with the derived encryption key and the dApp information received
        // in the original deep link.
        let expected_session = Session::new(
            SessionID(
                request_params.session_id.clone().unwrap().parse().unwrap(),
            ),
            SessionOrigin::WebDapp(DappOrigin::new(
                request_params.origin.clone().unwrap().to_string(),
            )),
            expected_encryption_key,
            dapp_public_key,
            Ed25519PublicKey::from_hex(
                request_params.identity_public_key.clone().unwrap(),
            )
            .unwrap(),
            wallet_public_key,
        );

        // Assert that the proper session was created
        pretty_assertions::assert_eq!(created_session, expected_session);

        // Assert that no session was saved to session storage yet,
        // it will be saved once the user sends back a successful response.
        pretty_assertions::assert_eq!(
            mock_session_storage.sessions.lock().unwrap().len(),
            0
        );
    }

    // Test that the proper request to be handled by the Host is returned when the dApp sends the first request.
    // One of the most important things is that the origin needs to be validated by the user.
    #[actix_rt::test]
    async fn test_happy_path_no_prior_session_proper_request_is_returned() {
        let sut = SUT::init(
            Arc::new(MockWalletInteractionTransport::succeeding()),
            Arc::new(MockSessionStorage::new()),
        );

        let request_params = SampleRequestParams::new_from_text_vector();
        let request = sut
            .handle_deep_link(request_params.build_base_url().to_string())
            .await
            .unwrap();

        let expected_request = RadixConnectMobileSessionRequest::new(
            SessionID(
                request_params.session_id.clone().unwrap().parse().unwrap(),
            ),
            SampleRequestParams::test_vector_encoded_interaction(),
            DappOrigin::new(request_params.origin.clone().unwrap()),
            true, // The origin needs to be validated by the user as there is no other session stored
        );

        pretty_assertions::assert_eq!(request, expected_request);
    }

    // Test that the in flight session will not be saved to session storage if the first wallet interacion is failure
    #[actix_rt::test]
    async fn test_happy_path_no_prior_session_failed_wallet_interaction() {
        let mock_transport =
            Arc::new(MockWalletInteractionTransport::succeeding());
        let mock_session_storage = Arc::new(MockSessionStorage::new());
        let sut =
            SUT::init(mock_transport.clone(), mock_session_storage.clone());

        let request_params = SampleRequestParams::new_from_text_vector();
        let request = sut
            .handle_deep_link(request_params.build_base_url().to_string())
            .await
            .unwrap();

        // Create a response to be sent back to the dApp
        let interaction_response = WalletToDappInteractionResponse::Failure(
            WalletToDappInteractionFailureResponse::sample_with_id(
                request.interaction.interaction_id.clone(),
            ),
        );
        let wallet_response = RadixConnectMobileWalletResponse::new(
            request.session_id,
            interaction_response.clone(),
        );

        let in_flight_session = sut
            .new_sessions
            .read()
            .unwrap()
            .get(&request.session_id)
            .unwrap()
            .get(&request.interaction.interaction_id)
            .cloned()
            .unwrap();

        // Send the response back to the dApp
        sut.send_dapp_interaction_response(wallet_response.clone())
            .await
            .unwrap();

        // Assert that the session was not saved to session storage
        pretty_assertions::assert_eq!(
            mock_session_storage.sessions.lock().unwrap().len(),
            0
        );

        // Assert that a response was sent to the dApp
        pretty_assertions::assert_eq!(
            mock_transport.responses.lock().unwrap().len(),
            1
        );

        // Assert that transport was called with the proper session and response
        pretty_assertions::assert_eq!(
            (in_flight_session, interaction_response),
            mock_transport
                .responses
                .lock()
                .unwrap()
                .first()
                .unwrap()
                .clone()
        );

        // Assert that the in flight session was removed from in flight sessions
        pretty_assertions::assert_eq!(
            sut.new_sessions.read().unwrap().len(),
            0
        );
    }

    // Test that the in flight session will not be saved to session storage if the first wallet interacion is failure
    #[actix_rt::test]
    async fn test_happy_path_no_prior_session_failed_to_send_response() {
        let response_send_error = Err(CommonError::Unknown {
            error_message: "Error".to_string(),
        });
        let mock_transport =
            Arc::new(MockWalletInteractionTransport::new(response_send_error));
        let mock_session_storage = Arc::new(MockSessionStorage::new());
        let sut =
            SUT::init(mock_transport.clone(), mock_session_storage.clone());

        let request_params = SampleRequestParams::new_from_text_vector();
        let request = sut
            .handle_deep_link(request_params.build_base_url().to_string())
            .await
            .unwrap();

        // Create a response to be sent back to the dApp
        let interaction_response = WalletToDappInteractionResponse::sample();
        let wallet_response = RadixConnectMobileWalletResponse::new(
            request.session_id,
            interaction_response.clone(),
        );

        // Send the response back to the dApp
        let result = sut
            .send_dapp_interaction_response(wallet_response.clone())
            .await;

        pretty_assertions::assert_eq!(result.is_err(), true);

        // Assert that the session was not saved to session storage
        pretty_assertions::assert_eq!(
            mock_session_storage.sessions.lock().unwrap().len(),
            0
        );
        // Assert that the in flight session was not removed from in flight sessions
        // On the Host side we do allow the user to retry the interaction
        pretty_assertions::assert_eq!(
            sut.new_sessions.read().unwrap().len(),
            1
        );
    }

    // Test that the in flight session will be saved to session storage if the first wallet interacion is success
    #[actix_rt::test]
    async fn test_happy_path_no_prior_session_user_success_wallet_interaction()
    {
        let mock_transport =
            Arc::new(MockWalletInteractionTransport::succeeding());
        let mock_session_storage = Arc::new(MockSessionStorage::new());
        let sut =
            SUT::init(mock_transport.clone(), mock_session_storage.clone());

        let request_params = SampleRequestParams::new_from_text_vector();
        let request = sut
            .handle_deep_link(request_params.build_base_url().to_string())
            .await
            .unwrap();

        let in_flight_session = sut
            .new_sessions
            .read()
            .unwrap()
            .get(&request.session_id)
            .unwrap()
            .get(&request.interaction.interaction_id)
            .cloned()
            .unwrap();

        // Create a response to be sent back to the dApp
        let interaction_response = WalletToDappInteractionResponse::Success(
            WalletToDappInteractionSuccessResponse::sample_with_id(
                request.interaction.interaction_id.clone(),
            ),
        );
        let wallet_response = RadixConnectMobileWalletResponse::new(
            request.session_id,
            interaction_response.clone(),
        );

        // Send the response back to the dApp
        sut.send_dapp_interaction_response(wallet_response.clone())
            .await
            .unwrap();

        // Assert that the session was saved to session storage as the user approved the request
        pretty_assertions::assert_eq!(
            mock_session_storage.sessions.lock().unwrap().len(),
            1
        );
        pretty_assertions::assert_eq!(
            mock_session_storage
                .sessions
                .lock()
                .unwrap()
                .get(&request.session_id)
                .unwrap(),
            &in_flight_session
        );

        // Assert that a response was sent to the dApp
        pretty_assertions::assert_eq!(
            mock_transport.responses.lock().unwrap().len(),
            1
        );

        // Assert that transport was called with the proper session and response
        pretty_assertions::assert_eq!(
            (in_flight_session, interaction_response),
            mock_transport
                .responses
                .lock()
                .unwrap()
                .first()
                .unwrap()
                .clone()
        );

        // Assert that the in flight session was removed from in flight sessions
        pretty_assertions::assert_eq!(
            sut.new_sessions.read().unwrap().len(),
            0
        );
    }

    // Test that after a session is established, the stored session is used for the new dApp requests.
    #[actix_rt::test]
    async fn test_happy_path_existing_session_flow() {
        let mock_transport =
            Arc::new(MockWalletInteractionTransport::succeeding());
        let mock_session_storage = Arc::new(MockSessionStorage::new());
        let sut =
            SUT::init(mock_transport.clone(), mock_session_storage.clone());

        let request_params = SampleRequestParams::new_from_text_vector();
        sut.handle_deep_link(request_params.build_base_url().to_string())
            .await
            .unwrap();

        let session_id = SessionID(
            request_params.session_id.clone().unwrap().parse().unwrap(),
        );
        let response = WalletToDappInteractionResponse::Success(
            WalletToDappInteractionSuccessResponse::sample_with_id(
                SampleRequestParams::test_vector_encoded_interaction()
                    .interaction_id,
            ),
        );

        sut.send_dapp_interaction_response(
            RadixConnectMobileWalletResponse::new(session_id, response.clone()),
        )
        .await
        .unwrap();

        // At this point the session is stored in secure storage, send another dApp request
        let request = sut
            .handle_deep_link(request_params.build_base_url().to_string())
            .await
            .unwrap();

        // Assert that no new session was created
        pretty_assertions::assert_eq!(
            sut.new_sessions.read().unwrap().len(),
            0
        );

        // Assert that the proper response was returned, especially that the origin does not need to be validated
        pretty_assertions::assert_eq!(
            request,
            RadixConnectMobileSessionRequest::new(
                session_id,
                SampleRequestParams::test_vector_encoded_interaction(),
                DappOrigin::new(request_params.origin.clone().unwrap()),
                false, // The origin does not need to be validated as the session is already stored
            )
        );

        // Create a failure response to validate the already established session is still kept
        // even if the wallet interaction is failure
        let failure_response = WalletToDappInteractionResponse::Failure(
            WalletToDappInteractionFailureResponse::sample(),
        );

        sut.send_dapp_interaction_response(
            RadixConnectMobileWalletResponse::new(
                session_id,
                failure_response.clone(),
            ),
        )
        .await
        .unwrap();

        // Assert that new response was sent
        pretty_assertions::assert_eq!(
            mock_transport.responses.lock().unwrap().len(),
            2
        );

        // Assert that saved session was used to send the response
        let saved_session = mock_session_storage
            .sessions
            .lock()
            .unwrap()
            .get(&session_id)
            .cloned()
            .unwrap();
        pretty_assertions::assert_eq!(
            mock_transport
                .responses
                .lock()
                .unwrap()
                .get(1)
                .cloned()
                .unwrap(),
            (saved_session.clone(), failure_response)
        );
    }
}
