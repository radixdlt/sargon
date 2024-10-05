use crate::prelude::*;
use sargon::DappToWalletInteractionMetadataUnvalidated as InternalDappToWalletInteractionMetadataUnvalidated;

#[derive(Debug, PartialEq, uniffi::Record, Clone)]
pub struct DappToWalletInteractionMetadataUnvalidated {
    pub version: WalletInteractionVersion,
    pub network_id: NetworkID,
    pub origin: DappOrigin,
    pub dapp_definition_address: String,
}

impl From<InternalDappToWalletInteractionMetadataUnvalidated> for DappToWalletInteractionMetadataUnvalidated {
    fn from(value: InternalDappToWalletInteractionMetadataUnvalidated) -> Self {
        Self {
            version: value.version.into(),
            network_id: value.network_id.into(),
            origin: value.origin.into(),
            dapp_definition_address: value.dapp_definition_address,
        }
    }
}

impl Into<InternalDappToWalletInteractionMetadataUnvalidated> for DappToWalletInteractionMetadataUnvalidated {
    fn into(self) -> InternalDappToWalletInteractionMetadataUnvalidated {
        InternalDappToWalletInteractionMetadataUnvalidated {
            version: self.version.into(),
            network_id: self.network_id.into(),
            origin: self.origin.into(),
            dapp_definition_address: self.dapp_definition_address,
        }
    }
}
