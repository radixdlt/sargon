use crate::prelude::*;

use sargon::WalletInteractionWalletAccount as InternalWalletInteractionWalletAccount;

#[derive(PartialEq, Clone, InternalConversion, uniffi::Record)]
pub struct WalletInteractionWalletAccount {
    pub address: AccountAddress,
    pub label: DisplayName,
    pub appearance_id: AppearanceID,
}

impl From<InternalWalletInteractionWalletAccount>
    for WalletInteractionWalletAccount
{
    fn from(value: InternalWalletInteractionWalletAccount) -> Self {
        Self {
            address: value.address.into(),
            label: value.label.into(),
            appearance_id: value.appearance_id.into(),
        }
    }
}

impl Into<InternalWalletInteractionWalletAccount>
    for WalletInteractionWalletAccount
{
    fn into(self) -> InternalWalletInteractionWalletAccount {
        InternalWalletInteractionWalletAccount {
            address: self.address.into(),
            label: self.label.into(),
            appearance_id: self.appearance_id.into(),
        }
    }
}

json_data_convertible!(WalletInteractionWalletAccount);

#[uniffi::export]
pub fn new_wallet_interaction_wallet_account_sample(
) -> WalletInteractionWalletAccount {
    InternalWalletInteractionWalletAccount::sample().into()
}

#[uniffi::export]
pub fn new_wallet_interaction_wallet_account_sample_other(
) -> WalletInteractionWalletAccount {
    InternalWalletInteractionWalletAccount::sample_other().into()
}
