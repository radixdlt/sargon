use crate::prelude::*;
use sargon::DappToWalletInteractionMetadata as InternalDappToWalletInteractionMetadata;

#[derive( PartialEq,  uniffi::Record, Clone)]
pub struct DappToWalletInteractionMetadata {
    pub version: WalletInteractionVersion,
    pub network_id: NetworkID,
    pub origin: DappOrigin,
    pub dapp_definition_address: DappDefinitionAddress,
}

impl From<InternalDappToWalletInteractionMetadata> for DappToWalletInteractionMetadata {
    fn from(value: InternalDappToWalletInteractionMetadata) -> Self {
        Self {
            version: value.version.into(),
            network_id: value.network_id.into(),
            origin: value.origin.into(),
            dapp_definition_address: value.dapp_definition_address.into(),
        }
    }
}

impl Into<InternalDappToWalletInteractionMetadata> for DappToWalletInteractionMetadata {
    fn into(self) -> InternalDappToWalletInteractionMetadata {
        InternalDappToWalletInteractionMetadata {
            version: self.version.into(),
            network_id: self.network_id.into(),
            origin: self.origin.into(),
            dapp_definition_address: self.dapp_definition_address.into(),
        }
    }
}

