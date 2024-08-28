use crate::prelude::*;

json_data_convertible!(WalletInteractionWalletAccount);

#[uniffi::export]
pub fn new_wallet_interaction_wallet_account_sample(
) -> WalletInteractionWalletAccount {
    WalletInteractionWalletAccount::sample()
}

#[uniffi::export]
pub fn new_wallet_interaction_wallet_account_sample_other(
) -> WalletInteractionWalletAccount {
    WalletInteractionWalletAccount::sample_other()
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[test]
    fn sample_values() {
        assert_ne!(
            new_wallet_interaction_wallet_account_sample(),
            new_wallet_interaction_wallet_account_sample_other(),
        );
    }
}
