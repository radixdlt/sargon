use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct WalletInteractionWalletAccount {
    pub address: AccountAddress,
    pub label: String,
    pub appearance_id: AppearanceID,
}

impl WalletInteractionWalletAccount {
    pub fn new(
        address: impl Into<AccountAddress>,
        label: impl AsRef<str>,
        appearance_id: impl Into<AppearanceID>,
    ) -> Self {
        Self {
            address: address.into(),
            label: label.as_ref().to_owned(),
            appearance_id: appearance_id.into(),
        }
    }
}

impl HasSampleValues for WalletInteractionWalletAccount {
    fn sample() -> Self {
        Self::new(AccountAddress::sample(), "sample1", AppearanceID::sample())
    }

    fn sample_other() -> Self {
        Self::new(
            AccountAddress::sample_other(),
            "sample2",
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
