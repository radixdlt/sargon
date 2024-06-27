use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SuccessResponse {
    pub method: RadixRelayRequestMethod,
    /// The unique id of the session established with the dApp.
    pub session_id: SessionID,
    /// Wallet's public key to be used to create the shared secret with the dApp.
    pub public_key: KeyAgreementPublicKey,
    /// Hex encoded WalletInteractionResponse
    pub data: BagOfBytes,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RadixRelayRequestMethod {
    SendResponse,
}

impl SuccessResponse {
    pub fn new(
        session_id: SessionID,
        wallet_public_key: KeyAgreementPublicKey,
        interaction_response: BagOfBytes,
    ) -> Self {
        Self {
            method: RadixRelayRequestMethod::SendResponse,
            session_id,
            public_key: wallet_public_key,
            data: interaction_response,
        }
    }
}

impl HasSampleValues for SuccessResponse {
    fn sample() -> Self {
        Self::new(
            SessionID::sample(),
            KeyAgreementPublicKey::sample(),
            BagOfBytes::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            SessionID::sample_other(),
            KeyAgreementPublicKey::sample_other(),
            BagOfBytes::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SuccessResponse;

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
