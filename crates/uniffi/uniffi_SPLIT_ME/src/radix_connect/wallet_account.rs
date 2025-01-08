use crate::prelude::*;

use sargon::WalletInteractionWalletAccount as InternalWalletInteractionWalletAccount;

#[derive(PartialEq, Clone, InternalConversion, uniffi::Record)]
pub struct WalletInteractionWalletAccount {
    pub address: AccountAddress,
    pub label: DisplayName,
    pub appearance_id: AppearanceID,
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
