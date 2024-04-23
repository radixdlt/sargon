use crate::prelude::*;

#[derive(Debug, Deserialize, Serialize, PartialEq, uniffi::Record, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionMetadata {
    pub version: WalletInteractionVersion,
    pub network_id: NetworkID,
    pub origin: Url,
    #[serde(rename = "dAppDefinitionAddress")]
    pub dapp_definition_address: DappDefinitionAddress,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionMetadata;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
