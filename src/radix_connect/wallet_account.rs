use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct WalletInteractionWalletAccount {
    pub address: AccountAddress,
    pub label: String,
    pub appearance_id: AppearanceID,
}

impl HasSampleValues for WalletInteractionWalletAccount {
    fn sample() -> Self {
        Self {
            address: AccountAddress::sample(),
            label: "sample1".to_string(),
            appearance_id: AppearanceID::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            address: AccountAddress::sample_other(),
            label: "sample2".to_string(),
            appearance_id: AppearanceID::sample_other(),
        }
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
