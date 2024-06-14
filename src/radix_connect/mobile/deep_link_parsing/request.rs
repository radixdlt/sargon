use crypto::signatures::ed25519::Signature;

use super::super::session::session_id::SessionID;
use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub struct RadixConnectMobileRequest {
    pub session_id: SessionID,
    pub origin: Url,
    pub public_key: KeyAgreementPublicKey,
    pub identity_public_key: Ed25519PublicKey,
    pub dapp_definition_address: DappDefinitionAddress,
    pub signature: Ed25519Signature,
    pub request: DappToWalletInteractionUnvalidated,
}

impl RadixConnectMobileRequest {
    pub fn new(
        session_id: SessionID,
        origin: Url,
        public_key: KeyAgreementPublicKey,
        identity_public_key: Ed25519PublicKey,
        dapp_definition_address: DappDefinitionAddress,
        signature: Ed25519Signature,
        request: DappToWalletInteractionUnvalidated,
    ) -> Self {
        Self {
            session_id,
            origin,
            public_key,
            identity_public_key,
            dapp_definition_address,
            signature,
            request,
        }
    }
}

impl HasSampleValues for RadixConnectMobileRequest {
    fn sample() -> Self {
        RadixConnectMobileRequest::new(
            SessionID::sample(),
            Url::parse("https://sample.com").unwrap(),
            KeyAgreementPublicKey::sample(),
            Ed25519PublicKey::sample(),
            DappDefinitionAddress::sample(),
            Ed25519Signature::sample(),
            DappToWalletInteractionUnvalidated::sample(),
        )
    }

    fn sample_other() -> Self {
        RadixConnectMobileRequest::new(
            SessionID::sample(),
            Url::parse("https://sample.com").unwrap(),
            KeyAgreementPublicKey::sample(),
            Ed25519PublicKey::sample_other(),
            DappDefinitionAddress::sample(),
            Ed25519Signature::sample(),
            DappToWalletInteractionUnvalidated::sample(),
        )
    }
}
