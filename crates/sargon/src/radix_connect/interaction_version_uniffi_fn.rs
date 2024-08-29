use crate::prelude::*;

#[uniffi::export]
pub fn new_wallet_interaction_version_current() -> WalletInteractionVersion {
    WalletInteractionVersion::current()
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[test]
    fn current() {
        assert_eq!(
            new_wallet_interaction_version_current(),
            WalletInteractionVersion::current()
        )
    }
}
