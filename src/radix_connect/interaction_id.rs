use crate::prelude::*;

uniffi::custom_newtype!(WalletInteractionId, String);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WalletInteractionId(pub String);

impl WalletInteractionId {
    pub fn new(id: impl AsRef<str>) -> Self {
        Self(id.as_ref().to_owned())
    }
}

impl HasSampleValues for WalletInteractionId {
    fn sample() -> Self {
        Self::new("sample")
    }

    fn sample_other() -> Self {
        Self::new("sample_other")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = WalletInteractionId;

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
