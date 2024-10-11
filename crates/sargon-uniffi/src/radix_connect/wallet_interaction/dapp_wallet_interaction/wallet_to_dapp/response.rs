use crate::prelude::*;
use sargon::WalletToDappInteractionResponse as InternalWalletToDappInteractionResponse;

json_data_convertible!(WalletToDappInteractionResponse);

#[derive(Clone, PartialEq, InternalConversion, uniffi::Enum)]
#[allow(clippy::large_enum_variant)]
pub enum WalletToDappInteractionResponse {
    Success(WalletToDappInteractionSuccessResponse),
    Failure(WalletToDappInteractionFailureResponse),
}

impl From<InternalWalletToDappInteractionResponse>
    for WalletToDappInteractionResponse
{
    fn from(value: InternalWalletToDappInteractionResponse) -> Self {
        match value {
            InternalWalletToDappInteractionResponse::Success(value) => {
                WalletToDappInteractionResponse::Success(value.into())
            }
            InternalWalletToDappInteractionResponse::Failure(value) => {
                WalletToDappInteractionResponse::Failure(value.into())
            }
        }
    }
}

impl Into<InternalWalletToDappInteractionResponse>
    for WalletToDappInteractionResponse
{
    fn into(self) -> InternalWalletToDappInteractionResponse {
        match self {
            WalletToDappInteractionResponse::Success(value) => {
                InternalWalletToDappInteractionResponse::Success(value.into())
            }
            WalletToDappInteractionResponse::Failure(value) => {
                InternalWalletToDappInteractionResponse::Failure(value.into())
            }
        }
    }
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
