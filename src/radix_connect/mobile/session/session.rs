use super::session_id::SessionID;
use super::session_origin::SessionOrigin;
use crate::prelude::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Session {
    pub id: SessionID,
    pub origin: SessionOrigin,
    pub encryption_key: SymmetricKey,
    pub dapp_public_key: KeyAgreementPublicKey,
    pub dapp_identity_public_key: Ed25519PublicKey,
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
        if self.dapp_identity_public_key != request.identity_public_key {
            return Err(CommonError::RadixConnectMobileDappPublicKeyMismatch);
        }

        if self.dapp_public_key != request.public_key {
            return Err(CommonError::RadixConnectMobileDappIdentityMismatch);
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

        let private_key = KeyAgreementPrivateKey::try_from(Exactly32Bytes::from_hex("4181137e23935d9b0e2bc39a798817df6bdddaab415d604801ef76b412f48124").unwrap()).unwrap();
        let public_key = KeyAgreementPublicKey::from_hex(
            "3791118a7a07c7f577538dc7b60be64245f13e4e5e0f284a3de9b3c77f946571"
                .to_string(),
        )
        .unwrap();

        let shared_secret =
            private_key.shared_secret_from_key_agreement(&public_key);
        let shared_secret_bytes: Exactly32Bytes =
            shared_secret.as_bytes().into();

        pretty_assertions::assert_eq!(
            shared_secret_bytes.to_hex(),
            "6542370bfbc9d818a621095210c34e2d376cb7a3edefa39fe25d02733b708c0d"
        );

        let salt = "account_tdx_2_12yf9gd53yfep7a669fv2t3wm7nz9zeezwd04n02a433ker8vza6rhe";
        let info = "RCfM";
        let key = PbHkdfSha256::hkdf_key_agreement(
            &shared_secret.as_bytes(),
            Some(salt.as_bytes()),
            Some(info.as_bytes()),
        );

        pretty_assertions::assert_eq!(
            key.to_hex(),
            "e9278143a272e9cce596335d0f29e0194305a2a00b57501bc4c21a432d5c2b49"
        );
    }
}
