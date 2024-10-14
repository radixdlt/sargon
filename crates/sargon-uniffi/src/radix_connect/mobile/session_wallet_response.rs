use crate::prelude::*;
use sargon::RadixConnectMobileWalletResponse as InternalRadixConnectMobileWalletResponse;

#[derive(PartialEq, Clone, InternalConversionV2, uniffi::Record)]
pub struct RadixConnectMobileWalletResponse {
    pub session_id: SessionID,
    pub response: WalletToDappInteractionResponse,
}
