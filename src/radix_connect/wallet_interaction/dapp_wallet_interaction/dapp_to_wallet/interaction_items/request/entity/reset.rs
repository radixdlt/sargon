use crate::prelude::*;

#[derive(Debug, Deserialize, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionResetRequestItem {
    pub accounts: bool,
    pub persona_data: bool,
}

impl DappToWalletInteractionResetRequestItem {
    pub fn new(accounts: bool, persona_data: bool) -> Self {
        Self {
            accounts,
            persona_data,
        }
    }
}

impl HasSampleValues for DappToWalletInteractionResetRequestItem {
    fn sample() -> Self {
        Self::new(true, true)
    }

    fn sample_other() -> Self {
        Self::new(false, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionResetRequestItem;

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
