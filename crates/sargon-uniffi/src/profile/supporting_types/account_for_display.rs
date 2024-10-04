use crate::prelude::*;
use sargon::AccountForDisplay as InternalAccountForDisplay;

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
    uniffi::Record,
)]
#[display("{display_name} | {address}")]
pub struct AccountForDisplay {
    pub address: AccountAddress,

    #[serde(rename = "label")]
    pub display_name: DisplayName,

    #[serde(rename = "appearanceID")]
    pub appearance_id: AppearanceID,
}

impl From<InternalAccountForDisplay> for AccountForDisplay {
    fn from(value: InternalAccountForDisplay) -> Self {
        unimplemented!()
    }
}

impl Into<InternalAccountForDisplay> for AccountForDisplay {
    fn into(self) -> InternalAccountForDisplay {
        unimplemented!()
    }
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

impl From<Account> for AccountForDisplay {
    fn from(value: Account) -> Self {
        Self::new(value.address, value.display_name, value.appearance_id)
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

    #[test]
    fn from_account() {
        let lhs = SUT::from(Account::sample());
        assert_eq!(
            lhs,
            SUT::new(
                "account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87",
                DisplayName::new("Alice").unwrap(),
                AppearanceID::new(0).unwrap(),
            )
        )
    }
}
