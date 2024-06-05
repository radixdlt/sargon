use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DappDefinition {
    #[serde(rename = "dAppDefinitionAddress")]
    pub dapp_definition_address: AccountAddress,
}

impl HasSampleValues for DappDefinition {
    fn sample() -> Self {
        Self {
            dapp_definition_address: AccountAddress::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            dapp_definition_address: AccountAddress::sample_other(),
        }
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
