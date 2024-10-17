use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, uniffi::Record)]
pub struct DappToWalletInteractionPreAuthorizationItems {
    pub subintent: Option<DappToWalletInteractionSubintentRequestItem>,
}

impl DappToWalletInteractionPreAuthorizationItems {
    pub fn new(
        subintent: impl Into<Option<DappToWalletInteractionSubintentRequestItem>>,
    ) -> Self {
        Self {
            subintent: subintent.into(),
        }
    }
}

impl HasSampleValues for DappToWalletInteractionPreAuthorizationItems {
    fn sample() -> Self {
        Self::new(DappToWalletInteractionSubintentRequestItem::sample())
    }

    fn sample_other() -> Self {
        Self::new(DappToWalletInteractionSubintentRequestItem::sample_other())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionPreAuthorizationItems;

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
