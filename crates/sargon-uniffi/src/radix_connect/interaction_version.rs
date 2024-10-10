use crate::prelude::*;
use sargon::WalletInteractionVersion as InternalWalletInteractionVersion;


#[derive(Debug, PartialEq, Clone,  uniffi::Record)]
pub struct WalletInteractionVersion {
    pub value: u64,
}

impl From<InternalWalletInteractionVersion> for WalletInteractionVersion {
    fn from(value: InternalWalletInteractionVersion) -> Self {
        Self {
            value: value.0,
        }
    }
}

impl Into<InternalWalletInteractionVersion> for WalletInteractionVersion {
    fn into(self) -> InternalWalletInteractionVersion {
        InternalWalletInteractionVersion(self.value)
    }
}

#[uniffi::export]
pub fn new_wallet_interaction_version_current() -> WalletInteractionVersion {
    InternalWalletInteractionVersion::current().into()
}

