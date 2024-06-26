use super::session_id::SessionID;
use super::session_origin::SessionOrigin;
use crate::prelude::*;

/// A session established between a dApp and the Wallet.     
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Session {
    /// Uniquely identifies a session. This value is known to both the dApp and the Wallet.
    /// The dApp will send requests by specifying a given session_id and the Wallet will respond by uploading
    /// the response to the Radix Relay Server for the exact same session_id.
    /// A dApp my have multiple session ids across different browser.
    pub id: SessionID,
    /// The origin of the dApp this session is for. It is used first to inform users where a request comes from,
    /// and second to validate any subsequent request in a session. Once a session is established, for the same
    /// session_id the origin needs to be the same.
    pub origin: SessionOrigin,
    /// The encryption key shared by both the dApp and the Wallet, that allows the dApp to decrypt the responses sent
    /// by the Wallet over the Relay Server. The encryption key is unique per session id and is created after performing a Diffie Hellman key exchange.
    pub encryption_key: SymmetricKey,
    /// The dapp public key sent over with the request that was used to generate the encryption key.
    /// The main purpose of having this stored in the session, is verify the validity of any subsequent dApp requests
    /// for a given session. It is expected that this public should not change after a session is established,
    /// if it happens to be the case, then most likely a bad actor is trying to hijack the session - the Wallet will reject such requests.
    pub dapp_public_key: KeyAgreementPublicKey,
    /// The dapp identity public key that was sent over to verify the request signature.
    /// The main purpose of having this stored in the session, is to verify the validity of any subsequent dApp requests
    /// for a given session. It is expected that this public should not change after a session is established,
    /// if it happens to be the case, then most likely a bad actor is trying to hijack the session - the Wallet will reject such requests.
    pub dapp_identity_public_key: Ed25519PublicKey,
    /// The wallet's public key used to generate the encryption_key.
    /// It is kept in the session and then send along with all of the Wallet's request so that the dApp can regenerate the encryption key if needed.
    pub wallet_public_key: KeyAgreementPublicKey,
}

impl Session {
    pub fn new(
        id: impl Into<SessionID>,
        origin: SessionOrigin,
        encryption_key: impl Into<Exactly32Bytes>,
        dapp_public_key: KeyAgreementPublicKey,
        dapp_identity_public_key: Ed25519PublicKey,
        wallet_public_key: KeyAgreementPublicKey,
    ) -> Self {
        Self {
            id: id.into(),
            origin,
            encryption_key: encryption_key.into(),
            dapp_public_key,
            dapp_identity_public_key,
            wallet_public_key,
        }
    }
}

impl Session {
    pub fn validate_request(
        &self,
        request: &RadixConnectMobileRequest,
    ) -> Result<()> {
        if self.origin != SessionOrigin::WebDapp(request.origin.clone()) {
            return Err(CommonError::RadixConnectMobileDappOriginMismatch);
        }

        if self.dapp_identity_public_key != request.identity_public_key {
            return Err(CommonError::RadixConnectMobileDappIdentityMismatch);
        }

        if self.dapp_public_key != request.public_key {
            return Err(CommonError::RadixConnectMobileDappPublicKeyMismatch);
        }

        Ok(())
    }
}

impl HasSampleValues for Session {
    fn sample() -> Self {
        Self::new(
            SessionID::sample(),
            SessionOrigin::sample(),
            Exactly32Bytes::sample(),
            KeyAgreementPublicKey::sample(),
            Ed25519PublicKey::sample(),
            KeyAgreementPublicKey::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            SessionID::sample_other(),
            SessionOrigin::sample_other(),
            Exactly32Bytes::sample_other(),
            KeyAgreementPublicKey::sample(),
            Ed25519PublicKey::sample(),
            KeyAgreementPublicKey::sample(),
        )
    }
}

#[cfg(test)]
mod tests {
    use hex::ToHex;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Session;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn test_validate_request() {
        let sut = SUT::sample();
        assert!(&sut
            .validate_request(&RadixConnectMobileRequest::sample())
            .is_ok());

        let mut wrong_dapp_public_key_request =
            RadixConnectMobileRequest::sample();
        wrong_dapp_public_key_request.public_key =
            KeyAgreementPublicKey::sample_other();
        pretty_assertions::assert_eq!(
            sut.validate_request(&wrong_dapp_public_key_request),
            Err(CommonError::RadixConnectMobileDappPublicKeyMismatch)
        );

        let mut wrong_dapp_identity_request =
            RadixConnectMobileRequest::sample();
        wrong_dapp_identity_request.identity_public_key =
            Ed25519PublicKey::sample_other();
        pretty_assertions::assert_eq!(
            sut.validate_request(&wrong_dapp_identity_request),
            Err(CommonError::RadixConnectMobileDappIdentityMismatch)
        );

        let mut wrong_dapp_origin_request = RadixConnectMobileRequest::sample();
        wrong_dapp_origin_request.origin = DappOrigin::sample_other();
        pretty_assertions::assert_eq!(
            sut.validate_request(&wrong_dapp_origin_request),
            Err(CommonError::RadixConnectMobileDappOriginMismatch)
        );
    }
}
