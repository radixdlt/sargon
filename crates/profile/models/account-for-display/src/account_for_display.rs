use crate::prelude::*;

/// A minimal version of an [`Account`] meant for
/// display purposes within wallet
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    Hash,
    Eq,
    derive_more::Display,
)]
#[display("{display_name} | {address}")]
pub struct AccountForDisplay {
    pub address: AccountAddress,

    #[serde(rename = "label")]
    pub display_name: DisplayName,

    #[serde(rename = "appearanceID")]
    pub appearance_id: AppearanceID,
}

impl AccountForDisplay {
    pub fn new(
        address: impl Into<AccountAddress>,
        display_name: impl Into<DisplayName>,
        appearance_id: impl Into<AppearanceID>,
    ) -> Self {
        Self {
            address: address.into(),
            display_name: display_name.into(),
            appearance_id: appearance_id.into(),
        }
    }
}

impl HasSampleValues for AccountForDisplay {
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

impl Identifiable for AccountForDisplay {
    type ID = AccountAddress;

    fn id(&self) -> Self::ID {
        self.address
    }
}

impl IsNetworkAware for AccountForDisplay {
    fn network_id(&self) -> NetworkID {
        self.address.network_id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccountForDisplay;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn test_is_network_aware() {
        assert_eq!(SUT::sample().network_id(), NetworkID::Mainnet);
    }
}
