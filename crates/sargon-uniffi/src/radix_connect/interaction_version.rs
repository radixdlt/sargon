use crate::prelude::*;
use sargon::WalletInteractionVersion as InternalWalletInteractionVersion;

uniffi::custom_newtype!(WalletInteractionVersion, u64);

#[derive(PartialEq, Clone, InternalConversion)]
pub struct WalletInteractionVersion(pub u64);

#[uniffi::export]
pub fn new_wallet_interaction_version_current() -> WalletInteractionVersion {
    InternalWalletInteractionVersion::current().into()
}
