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
