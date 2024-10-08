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
    uniffi::Record,
)]
pub struct AccountForDisplay {
    pub address: AccountAddress,

    pub display_name: DisplayName,

    pub appearance_id: AppearanceID,
}

impl From<InternalAccountForDisplay> for AccountForDisplay {
    fn from(value: InternalAccountForDisplay) -> Self {
        Self {
            address: value.address.into(),
            display_name: value.display_name.into(),
            appearance_id: value.appearance_id.into(),
        }
    }
}

impl Into<InternalAccountForDisplay> for AccountForDisplay {
    fn into(self) -> InternalAccountForDisplay {
        InternalAccountForDisplay {
            address: self.address.into(),
            display_name: self.display_name.into(),
            appearance_id: self.appearance_id.into(),
        }
    }
}

#[uniffi::export]
pub fn new_account_for_display_sample() -> AccountForDisplay {
    InternalAccountForDisplay::sample().into()
}

#[uniffi::export]
pub fn new_account_for_display_sample_other() -> AccountForDisplay {
    InternalAccountForDisplay::sample_other().into()
}

#[uniffi::export]
pub fn new_account_for_display_from_account(
    account: Account,
) -> AccountForDisplay {
    InternalAccountForDisplay::from(account).into()
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
