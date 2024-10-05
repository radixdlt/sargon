use crate::prelude::*;
use sargon::AccountForDisplay as InternalAccountForDisplay;

/// A minimal version of an [`Account`] meant for
/// display purposes within wallet
#[derive(
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

#[uniffi::export]
pub fn new_account_for_display_sample() -> AccountForDisplay {
    AccountForDisplay::sample()
}

#[uniffi::export]
pub fn new_account_for_display_sample_other() -> AccountForDisplay {
    AccountForDisplay::sample_other()
}

#[uniffi::export]
pub fn new_account_for_display_from_account(
    account: Account,
) -> AccountForDisplay {
    AccountForDisplay::from(account)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccountForDisplay;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_account_for_display_sample(),
                new_account_for_display_sample_other(),
                // duplicates should get removed
                new_account_for_display_sample(),
                new_account_for_display_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_new_account_for_display_from_account() {
        let sut = Account::sample();
        assert_eq!(
            new_account_for_display_from_account(sut.clone()),
            AccountForDisplay::from(sut)
        );
    }
}
