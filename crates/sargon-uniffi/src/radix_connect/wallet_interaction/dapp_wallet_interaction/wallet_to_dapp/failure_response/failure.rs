use crate::prelude::*;
use sargon::WalletToDappInteractionFailureResponse as InternalWalletToDappInteractionFailureResponse;

#[derive(Clone, PartialEq, InternalConversionV2, uniffi::Record)]
pub struct WalletToDappInteractionFailureResponse {
    pub interaction_id: WalletInteractionId,
    pub error: DappWalletInteractionErrorType,
    pub message: Option<String>,
}