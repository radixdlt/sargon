use crate::prelude::*;
use sargon::AccountForDisplay as InternalAccountForDisplay;

/// A minimal version of an [`Account`] meant for
/// display purposes within wallet
#[derive(Clone, PartialEq, Hash, Eq, InternalConversionV2, uniffi::Record)]
pub struct AccountForDisplay {
    pub address: AccountAddress,

    pub display_name: DisplayName,

    pub appearance_id: AppearanceID,
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
    InternalAccountForDisplay::from(account.into_internal()).into()
}
