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
}
