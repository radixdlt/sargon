use crate::prelude::*;
use sargon::DappToWalletInteractionMetadata as InternalDappToWalletInteractionMetadata;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct DappToWalletInteractionMetadata {
    pub version: WalletInteractionVersion,
    pub network_id: NetworkID,
    pub origin: DappOrigin,
    pub dapp_definition_address: DappDefinitionAddress,
}

json_data_convertible!(DappToWalletInteractionMetadata);

#[uniffi::export]
pub fn new_dapp_to_wallet_interaction_metadata_sample(
) -> DappToWalletInteractionMetadata {
    InternalDappToWalletInteractionMetadata::sample().into()
}

#[uniffi::export]
pub fn new_dapp_to_wallet_interaction_metadata_sample_other(
) -> DappToWalletInteractionMetadata {
    InternalDappToWalletInteractionMetadata::sample_other().into()
}
