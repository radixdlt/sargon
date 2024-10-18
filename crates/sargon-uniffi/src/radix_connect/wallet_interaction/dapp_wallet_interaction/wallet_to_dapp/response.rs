use crate::prelude::*;
use sargon::WalletToDappInteractionResponse as InternalWalletToDappInteractionResponse;

json_data_convertible!(WalletToDappInteractionResponse);

#[derive(Clone, PartialEq, InternalConversion, uniffi::Enum)]
#[allow(clippy::large_enum_variant)]
pub enum WalletToDappInteractionResponse {
    Success(WalletToDappInteractionSuccessResponse),
    Failure(WalletToDappInteractionFailureResponse),
}

#[uniffi::export]
pub(crate) fn new_wallet_to_dapp_interaction_response_sample(
) -> WalletToDappInteractionResponse {
    InternalWalletToDappInteractionResponse::sample().into()
}

#[uniffi::export]
pub(crate) fn new_wallet_to_dapp_interaction_response_sample_other(
) -> WalletToDappInteractionResponse {
    InternalWalletToDappInteractionResponse::sample_other().into()
}
