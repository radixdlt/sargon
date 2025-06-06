use crate::prelude::*;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionMetadataUnvalidated {
    pub version: WalletInteractionVersion,
    pub network_id: NetworkID,
    pub origin: DappOrigin,
    #[serde(rename = "dAppDefinitionAddress")]
    pub dapp_definition_address: String,
}

impl DappToWalletInteractionMetadataUnvalidated {
    pub fn new(
        version: impl Into<WalletInteractionVersion>,
        network_id: impl Into<NetworkID>,
        origin: impl Into<DappOrigin>,
        dapp_definition_address: impl AsRef<str>,
    ) -> Self {
        Self {
            version: version.into(),
            network_id: network_id.into(),
            origin: origin.into(),
            dapp_definition_address: dapp_definition_address
                .as_ref()
                .to_owned(),
        }
    }
}

impl HasSampleValues for DappToWalletInteractionMetadataUnvalidated {
    fn sample() -> Self {
        Self::new(
            WalletInteractionVersion::sample(),
            NetworkID::Stokenet,
            "https://example.com",
            DappDefinitionAddress::sample().to_string(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            WalletInteractionVersion::sample_other(),
            NetworkID::Stokenet,
            "https://example.org",
            DappDefinitionAddress::sample_other().to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionMetadataUnvalidated;

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
