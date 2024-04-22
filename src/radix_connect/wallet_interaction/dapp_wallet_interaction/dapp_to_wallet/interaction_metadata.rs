use crate::prelude::*;

#[derive(Debug, Deserialize, PartialEq, uniffi::Record, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionMetadata {
    pub version: WalletInteractionVersion,
    pub network_id: NetworkID,
    pub origin: Url,
    #[serde(rename = "dAppDefinitionAddress")]
    pub dapp_definition_address: DappDefinitionAddress,
}

#[derive(Debug, Deserialize, PartialEq, uniffi::Record, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionMetadataUnvalidated {
    pub version: WalletInteractionVersion,
    pub network_id: NetworkID,
    pub origin: Url,
    #[serde(rename = "dAppDefinitionAddress")]
    pub dapp_definition_address: String,
}

impl HasSampleValues for DappToWalletInteractionMetadata {
    fn sample() -> Self {
        Self {
            version: WalletInteractionVersion::sample(),
            network_id: NetworkID::Stokenet,
            origin: Url::from_str("https://example.com").unwrap(),
            dapp_definition_address: DappDefinitionAddress::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            version: WalletInteractionVersion::sample_other(),
            network_id: NetworkID::Stokenet,
            origin: Url::from_str("https://example.org").unwrap(),
            dapp_definition_address: DappDefinitionAddress::sample_other(),
        }
    }
}
