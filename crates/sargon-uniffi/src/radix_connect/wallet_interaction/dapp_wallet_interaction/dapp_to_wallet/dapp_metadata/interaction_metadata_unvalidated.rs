use crate::prelude::*;
use sargon::DappToWalletInteractionMetadataUnvalidated as InternalDappToWalletInteractionMetadataUnvalidated;

#[derive(Clone, PartialEq, InternalConversionV2, uniffi::Record)]
pub struct DappToWalletInteractionMetadataUnvalidated {
    pub version: WalletInteractionVersion,
    pub network_id: NetworkID,
    pub origin: DappOrigin,
    pub dapp_definition_address: String,
}
