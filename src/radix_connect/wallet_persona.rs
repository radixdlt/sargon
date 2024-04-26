use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappWalletInteractionPersona {
    pub identity_address: IdentityAddress,
    pub label: String,
}

impl DappWalletInteractionPersona {
    pub fn new(
        identity_address: impl Into<IdentityAddress>,
        label: impl AsRef<str>,
    ) -> Self {
        Self {
            identity_address: identity_address.into(),
            label: label.as_ref().to_owned(),
        }
    }
}

impl HasSampleValues for DappWalletInteractionPersona {
    fn sample() -> Self {
        Self::new(IdentityAddress::sample(), "sample1")
    }

    fn sample_other() -> Self {
        Self::new(IdentityAddress::sample_other(), "sample2")
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
