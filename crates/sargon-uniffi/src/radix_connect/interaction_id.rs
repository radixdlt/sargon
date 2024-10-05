use crate::prelude::*;
use sargon::WalletInteractionId as InternalWalletInteractionId;

uniffi::custom_newtype!(WalletInteractionId, String);

/// We ought to make `WalletInteractionId` a UUID.
/// Temporarily, it will be a String because the iOS wallet has specific logic that uses custom IDs for wallet interactions.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    Hash,
    derive_more::Display,
)]
pub struct WalletInteractionId(pub String);

impl From<InternalWalletInteractionId> for WalletInteractionId {
    fn from(value: InternalWalletInteractionId) -> Self {
        Self(value.0)
    }
}

impl Into<InternalWalletInteractionId> for WalletInteractionId {
    fn into(self) -> InternalWalletInteractionId {
        InternalWalletInteractionId(self.0)
    }
}