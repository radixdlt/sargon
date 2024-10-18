use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionAuthUsePersonaRequestItem {
    pub identity_address: IdentityAddress,
}

impl DappToWalletInteractionAuthUsePersonaRequestItem {
    pub fn new(identity_address: impl Into<IdentityAddress>) -> Self {
        Self {
            identity_address: identity_address.into(),
        }
    }
}

impl HasSampleValues for DappToWalletInteractionAuthUsePersonaRequestItem {
    fn sample() -> Self {
        Self::new(IdentityAddress::sample())
    }

    fn sample_other() -> Self {
        Self::new(IdentityAddress::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionAuthUsePersonaRequestItem;

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
