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

impl DappToWalletInteractionMetadata {
    pub fn new(
        version: impl Into<WalletInteractionVersion>,
        network_id: impl Into<NetworkID>,
        origin: impl Into<Url>,
        dapp_definition_address: impl Into<DappDefinitionAddress>,
    ) -> Self {
        Self {
            version: version.into(),
            network_id: network_id.into(),
            origin: origin.into(),
            dapp_definition_address: dapp_definition_address.into(),
        }
    }
}

impl HasSampleValues for DappToWalletInteractionMetadata {
    fn sample() -> Self {
        Self::new(
            WalletInteractionVersion::sample(),
            NetworkID::Stokenet,
            Url::from_str("https://example.com").unwrap(),
            DappDefinitionAddress::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            WalletInteractionVersion::sample_other(),
            NetworkID::Stokenet,
            Url::from_str("https://example.org").unwrap(),
            DappDefinitionAddress::sample_other(),
        )
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
