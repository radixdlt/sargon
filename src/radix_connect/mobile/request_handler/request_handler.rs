use super::super::relay_service::Service as RelayService;
use super::super::session::*;
use crate::{prelude::*, wallet};

pub struct RequestHandler {
    pub relay_service: RelayService,
    pub wallet_client_storage: WalletClientStorage,
}

impl RequestHandler {
    pub fn new(
        relay_service: RelayService,
        wallet_client_storage: WalletClientStorage,
    ) -> Self {
        Self {
            relay_service,
            wallet_client_storage,
        }
    }
}

pub struct RadixConnectMobileSessionRequest {
    pub session_id: SessionID,
    pub interaction: DappToWalletInteractionUnvalidated,
}

pub struct RadixConnectMobileWalletResponse {
    pub session_id: SessionID,
    pub response: WalletToDappInteractionResponse,
}

impl RequestHandler {
    pub async fn handle_dapp_request(
        &self,
        link_request: RadixConnectMobileDappRequest,
    ) -> Result<RadixConnectMobileSessionRequest> {
        let session = self
            .wallet_client_storage
            .load_session(link_request.session_id)?;

        let session = match session {
            Some(session) => session,
            None => {
                return Err(CommonError::RadixConnectMobileSessionNotFound {
                    session_id: link_request.session_id.0.to_string(),
                });
            }
        };

        let request = self
            .relay_service
            .get_wallet_interaction_requests(session)
            .await?
            .into_iter()
            .find(|dapp_request| {
                dapp_request.interaction_id == link_request.interaction_id
            })
            .ok_or_else(|| {
                CommonError::RadixConnectMobileDappRequestNotFound {
                    interaction_id: link_request.interaction_id.0.to_string(),
                }
            })?;

        Ok(RadixConnectMobileSessionRequest {
            session_id: link_request.session_id,
            interaction: request,
        })
    }

    pub async fn send_wallet_response(
        &self,
        wallet_response: RadixConnectMobileWalletResponse,
    ) -> Result<Url> {
        let session = self
            .wallet_client_storage
            .load_session(wallet_response.session_id)?;

        let session = match session {
            Some(session) => session,
            None => {
                return Err(CommonError::RadixConnectMobileSessionNotFound {
                    session_id: wallet_response.session_id.0.to_string(),
                });
            }
        };

        self.relay_service
            .send_wallet_interaction_response(session, wallet_response.response)
            .await?;

        Ok(Url::from_str("https://example.com").unwrap())
    }
}

impl WalletClientStorage {
    fn load_session(&self, session_id: SessionID) -> Result<Option<Session>> {
        self.load(SecureStorageKey::RadixConnectMobileSession { session_id })
    }
}

// TBA tests
