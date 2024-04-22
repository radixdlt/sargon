use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappWalletInteractionPersona {
    pub identity_address: IdentityAddress,
    pub label: String,
}

impl HasSampleValues for DappWalletInteractionPersona {
    fn sample() -> Self {
        Self {
            identity_address: IdentityAddress::sample(),
            label: "sample1".to_string(),
        }
    }

    fn sample_other() -> Self {
        Self {
            identity_address: IdentityAddress::sample_other(),
            label: "sample2".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappWalletInteractionPersona;

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