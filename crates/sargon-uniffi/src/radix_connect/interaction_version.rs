use crate::prelude::*;
use sargon::WalletInteractionVersion as InternalWalletInteractionVersion;

uniffi::custom_newtype!(WalletInteractionVersion, u64);

#[derive(Debug, PartialEq, Clone)]
pub struct WalletInteractionVersion(pub u64);

impl From<InternalWalletInteractionVersion> for WalletInteractionVersion {
    fn from(value: InternalWalletInteractionVersion) -> Self {
        Self(value.0)
    }
}

impl Into<InternalWalletInteractionVersion> for WalletInteractionVersion {
    fn into(self) -> InternalWalletInteractionVersion {
        InternalWalletInteractionVersion(self.0)
    }
}

#[uniffi::export]
pub fn new_wallet_interaction_version_current() -> WalletInteractionVersion {
    InternalWalletInteractionVersion::current().into()
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[test]
    fn current() {
        assert_eq!(
            new_wallet_interaction_version_current(),
            InternalWalletInteractionVersion::current().into()
        )
    }
}
