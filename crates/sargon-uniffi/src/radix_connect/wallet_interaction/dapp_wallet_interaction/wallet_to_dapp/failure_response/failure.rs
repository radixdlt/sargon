use crate::prelude::*;
use sargon::WalletToDappInteractionFailureResponse as InternalWalletToDappInteractionFailureResponse;

#[derive( Clone, PartialEq,  uniffi::Record)]
pub struct WalletToDappInteractionFailureResponse {
    pub interaction_id: WalletInteractionId,
    pub error: DappWalletInteractionErrorType,
    pub message: Option<String>,
}

impl From<InternalWalletToDappInteractionFailureResponse> for WalletToDappInteractionFailureResponse {
    fn from(value: InternalWalletToDappInteractionFailureResponse) -> Self {
        Self {
            interaction_id: value.interaction_id.into(),
            error: value.error.into(),
            message: value.message,
        }
    }
}

impl Into<InternalWalletToDappInteractionFailureResponse> for WalletToDappInteractionFailureResponse {
    fn into(self) -> InternalWalletToDappInteractionFailureResponse {
        InternalWalletToDappInteractionFailureResponse {
            interaction_id: self.interaction_id.into(),
            error: self.error.into(),
            message: self.message,
        }
    }
}
