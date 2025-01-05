use crate::prelude::*;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionMetadata {
    pub version: WalletInteractionVersion,
    pub network_id: NetworkID,
    pub origin: DappOrigin,
    #[serde(rename = "dAppDefinitionAddress")]
    pub dapp_definition_address: DappDefinitionAddress,
}

impl DappToWalletInteractionMetadata {
    pub fn new(
        version: impl Into<WalletInteractionVersion>,
        network_id: impl Into<NetworkID>,
        origin: impl Into<DappOrigin>,
        dapp_definition_address: impl Into<DappDefinitionAddress>,
    ) -> Self {
        Self {
            version: version.into(),
            network_id: network_id.into(),
            origin: origin.into(),
            dapp_definition_address: dapp_definition_address.into(),
        }
    }

    pub fn with_updated_origin(self, origin: impl Into<DappOrigin>) -> Self {
        Self {
            origin: origin.into(),
            ..self
        }
    }
}

impl HasSampleValues for DappToWalletInteractionMetadata {
    fn sample() -> Self {
        Self::new(
            WalletInteractionVersion::sample(),
            NetworkID::Mainnet,
            "https://example.com",
            DappDefinitionAddress::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            WalletInteractionVersion::sample_other(),
            NetworkID::Mainnet,
            "https://example.org",
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

    #[test]
    fn with_updated_origin() {
        let metadata = SUT::sample();
        let new_origin = DappOrigin::new("https://example.org");
        let metadata = metadata.with_updated_origin(new_origin.clone());
        assert_eq!(metadata.origin, new_origin);
    }
}
