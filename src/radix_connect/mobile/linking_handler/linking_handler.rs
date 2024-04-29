use super::super::relay_service::Service as RelayService;
use super::super::session::*;
use crate::{prelude::*, wallet};

pub struct LinkingHandler {
    pub relay_service: RelayService,
    pub wallet_client_storage: WalletClientStorage,
}

impl LinkingHandler {
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

impl LinkingHandler {
    pub async fn handle_link_request(
        &self,
        link_request: RadixConnectMobileLinkRequest,
    ) -> Result<Url> {
        let handshake_request = self
            .relay_service
            .get_session_handshake_request(link_request.session_id)
            .await?;

        let wallet_private_key = DiffieHellmanPrivateKey::generate()?;
        // random salt
        let salt = hex_decode("000102030405060708090a0b0c0d0e0f").unwrap();
        let info = hex_decode("f0f1f2f3f4f5f6f7f8f9").unwrap();
        let encryption_key = wallet_private_key.hkdf_key_agreement(
            &handshake_request.public_key,
            &salt,
            &info,
        )?;

        let session = Session::new(
            link_request.session_id,
            SessionOrigin::WebDapp(link_request.origin),
            encryption_key,
        );

        self.wallet_client_storage.save_session(session)?;

        Ok(Url::from_str("https://example.com").unwrap())
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
}

// TBD tests
