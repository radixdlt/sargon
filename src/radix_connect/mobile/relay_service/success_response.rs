use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SuccessResponse {
    pub method: String,
    pub session_id: SessionID,
    pub public_key: KeyAgreementPublicKey,
    pub data: String,
}

impl SuccessResponse {
    pub fn new(
        session_id: SessionID,
        wallet_public_key: KeyAgreementPublicKey,
        interaction_response: String,
    ) -> Self {
        Self {
            method: "sendResponse".to_owned(),
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
            "data".to_string(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            SessionID::sample_other(),
            KeyAgreementPublicKey::sample_other(),
            "data_other".to_string(),
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
