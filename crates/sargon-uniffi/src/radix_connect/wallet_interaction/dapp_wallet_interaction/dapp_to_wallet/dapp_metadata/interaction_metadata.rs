use crate::prelude::*;
use sargon::DappToWalletInteractionMetadata as InternalDappToWalletInteractionMetadata;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct DappToWalletInteractionMetadata {
    pub version: WalletInteractionVersion,
    pub network_id: NetworkID,
    pub origin: DappOrigin,
    pub dapp_definition_address: DappDefinitionAddress,
}