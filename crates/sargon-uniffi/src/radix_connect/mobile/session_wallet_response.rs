use crate::prelude::*;
use sargon::RadixConnectMobileWalletResponse as InternalRadixConnectMobileWalletResponse;

#[derive(Debug, PartialEq, Clone,  uniffi::Record)]
pub struct RadixConnectMobileWalletResponse {
    pub session_id: SessionID,
    pub response: WalletToDappInteractionResponse,
}

impl From<InternalRadixConnectMobileWalletResponse> for RadixConnectMobileWalletResponse {
    fn from(value: InternalRadixConnectMobileWalletResponse) -> Self {
        Self {
            session_id: value.session_id.into(),
            response: value.response.into(),
        }
    }
}

impl Into<InternalRadixConnectMobileWalletResponse> for RadixConnectMobileWalletResponse {
    fn into(self) -> InternalRadixConnectMobileWalletResponse {
        InternalRadixConnectMobileWalletResponse {
            session_id: self.session_id.into(),
            response: self.response.into(),
        }
    }
}