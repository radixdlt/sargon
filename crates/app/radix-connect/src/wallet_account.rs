use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WalletInteractionWalletAccount {
    pub address: AccountAddress,
    pub label: DisplayName,
    pub appearance_id: AppearanceID,
}

impl WalletInteractionWalletAccount {
    pub fn new(
        address: impl Into<AccountAddress>,
        label: impl Into<DisplayName>,
        appearance_id: impl Into<AppearanceID>,
    ) -> Self {
        Self {
            address: address.into(),
            label: label.into(),
            appearance_id: appearance_id.into(),
        }
    }
}

impl HasSampleValues for WalletInteractionWalletAccount {
    fn sample() -> Self {
        Self::new(
            AccountAddress::sample(),
            DisplayName::sample(),
            AppearanceID::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            AccountAddress::sample_other(),
            DisplayName::sample_other(),
            AppearanceID::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = WalletInteractionWalletAccount;

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
