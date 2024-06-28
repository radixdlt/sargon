use crate::prelude::*;

/// Wrapper around the address of a dapp.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DappDefinition {
    /// Address of the dapp.
    #[serde(rename = "dAppDefinitionAddress")]
    pub dapp_definition_address: DappDefinitionAddress,
}

impl DappDefinition {
    pub fn new(dapp_definition_address: AccountAddress) -> Self {
        Self {
            dapp_definition_address,
        }
    }
}

impl HasSampleValues for DappDefinition {
    fn sample() -> Self {
        Self::new(AccountAddress::sample())
    }

    fn sample_other() -> Self {
        Self::new(AccountAddress::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappDefinition;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
